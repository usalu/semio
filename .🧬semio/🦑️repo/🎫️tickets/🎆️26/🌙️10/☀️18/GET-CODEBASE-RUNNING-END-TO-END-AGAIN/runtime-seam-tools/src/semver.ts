interface VersionV1 { readonly major: number; readonly minor: number; readonly patch: number; readonly pre: readonly (string | number)[] }
type OperatorV1 = "<" | "<=" | ">" | ">=" | "=";
interface ComparatorV1 { readonly operator: OperatorV1; readonly version: VersionV1 }

const VERSION = /^[=v]*(\d+)\.(\d+)\.(\d+)(?:-([0-9A-Za-z.-]+))?(?:\+[0-9A-Za-z.-]+)?$/u;
const PARTIAL = /^[=v]*(x|X|\*|\d+)(?:\.(x|X|\*|\d+)(?:\.(x|X|\*|\d+)(?:-([0-9A-Za-z.-]+))?(?:\+[0-9A-Za-z.-]+)?)?)?$/u;

function parseVersion(text: string): VersionV1 | undefined {
  const match = VERSION.exec(text.trim());
  if (match === null) return undefined;
  return { major: Number(match[1]), minor: Number(match[2]), patch: Number(match[3]), pre: match[4] === undefined ? [] : match[4].split(".").map(part => (/^\d+$/u.test(part) ? Number(part) : part)) };
}

function compareIdentifiers(left: string | number, right: string | number): number {
  if (typeof left === "number" && typeof right === "number") return left < right ? -1 : left > right ? 1 : 0;
  if (typeof left === "number") return -1;
  if (typeof right === "number") return 1;
  return left < right ? -1 : left > right ? 1 : 0;
}

function compareVersions(left: VersionV1, right: VersionV1): -1 | 0 | 1 {
  for (const field of ["major", "minor", "patch"] as const) if (left[field] !== right[field]) return left[field] < right[field] ? -1 : 1;
  if (left.pre.length === 0 && right.pre.length === 0) return 0;
  if (left.pre.length === 0) return 1;
  if (right.pre.length === 0) return -1;
  for (let index = 0; index < Math.max(left.pre.length, right.pre.length); index++) {
    const a = left.pre[index], b = right.pre[index];
    if (a === undefined) return -1;
    if (b === undefined) return 1;
    const order = compareIdentifiers(a, b);
    if (order !== 0) return order as -1 | 1;
  }
  return 0;
}

const make = (major: number, minor: number, patch: number, pre: readonly (string | number)[] = []): VersionV1 => ({ major, minor, patch, pre });
const floor = (version: VersionV1): ComparatorV1 => ({ operator: ">=", version });
const below = (major: number, minor: number, patch: number): ComparatorV1 => ({ operator: "<", version: make(major, minor, patch, [0]) });

interface PartialV1 { readonly major: number | undefined; readonly minor: number | undefined; readonly patch: number | undefined; readonly pre: readonly (string | number)[] }

function parsePartial(text: string): PartialV1 | undefined {
  const match = PARTIAL.exec(text.trim());
  if (match === null) return undefined;
  const number_ = (value: string | undefined): number | undefined => (value === undefined || /^[xX*]$/u.test(value) ? undefined : Number(value));
  const major = number_(match[1]), minor = major === undefined ? undefined : number_(match[2]), patch = minor === undefined ? undefined : number_(match[3]);
  return { major, minor, patch, pre: patch === undefined || match[4] === undefined ? [] : match[4].split(".").map(part => (/^\d+$/u.test(part) ? Number(part) : part)) };
}

function lower(partial: PartialV1): VersionV1 { return make(partial.major ?? 0, partial.minor ?? 0, partial.patch ?? 0, partial.pre); }

function caret(partial: PartialV1): ComparatorV1[] {
  const { major, minor, patch } = partial;
  if (major === undefined) return [];
  const start = floor(lower(partial));
  if (major > 0 || minor === undefined) return [start, below(major + 1, 0, 0)];
  if (minor > 0 || patch === undefined) return [start, below(0, minor + 1, 0)];
  return [start, below(0, 0, patch + 1)];
}

function tilde(partial: PartialV1): ComparatorV1[] {
  const { major, minor } = partial;
  if (major === undefined) return [];
  return [floor(lower(partial)), minor === undefined ? below(major + 1, 0, 0) : below(major, minor + 1, 0)];
}

function xrange(operator: string, partial: PartialV1): ComparatorV1[] {
  const { major, minor, patch } = partial;
  if (major === undefined) return operator === "<" || operator === ">" ? [{ operator: "<", version: make(0, 0, 0, [0]) }] : [];
  if (operator === "" || operator === "=") {
    if (patch !== undefined) return [{ operator: "=", version: lower(partial) }];
    return [floor(lower(partial)), minor === undefined ? below(major + 1, 0, 0) : below(major, minor + 1, 0)];
  }
  if (operator === ">") { if (patch !== undefined) return [{ operator: ">", version: lower(partial) }]; return [floor(minor === undefined ? make(major + 1, 0, 0) : make(major, minor + 1, 0))]; }
  if (operator === ">=") return [floor(lower(partial))];
  if (operator === "<") return [{ operator: "<", version: lower(partial) }];
  if (patch !== undefined) return [{ operator: "<=", version: lower(partial) }];
  return [minor === undefined ? below(major + 1, 0, 0) : below(major, minor + 1, 0)];
}

function parseSet(text: string): ComparatorV1[] | undefined {
  let source = text.trim().replace(/\s+/gu, " ").replace(/(>=|<=|>|<|\^|~>?|=)\s+/gu, "$1");
  const hyphen = /^(\S+) - (\S+)$/u.exec(source);
  if (hyphen !== null) {
    const start = parsePartial(hyphen[1]!), end = parsePartial(hyphen[2]!);
    if (start === undefined || end === undefined) return undefined;
    return [...(start.major === undefined ? [] : [floor(lower(start))]), ...xrange("<=", end)];
  }
  const comparators: ComparatorV1[] = [];
  for (const token of source === "" ? ["*"] : source.split(" ")) {
    const match = /^(>=|<=|>|<|\^|~>?|=)?(.*)$/u.exec(token);
    const partial = parsePartial(match![2]!);
    if (partial === undefined) return undefined;
    const operator = match![1] ?? "";
    comparators.push(...(operator === "^" ? caret(partial) : operator === "~" || operator === "~>" ? tilde(partial) : xrange(operator, partial)));
  }
  return comparators;
}

function holds(version: VersionV1, comparator: ComparatorV1): boolean {
  const order = compareVersions(version, comparator.version);
  return comparator.operator === "=" ? order === 0 : comparator.operator === "<" ? order < 0 : comparator.operator === "<=" ? order <= 0 : comparator.operator === ">" ? order > 0 : order >= 0;
}

/** 🔢️ Decides whether a version satisfies an npm-style range (caret, tilde, x-ranges, hyphens, comparators, `||`); invalid input is simply false. */
export function semverSatisfiesV1(version: string, range: string): boolean {
  const parsed = parseVersion(version);
  if (parsed === undefined) return false;
  for (const alternative of range.split("||")) {
    const set = parseSet(alternative);
    if (set === undefined || !set.every(comparator => holds(parsed, comparator))) continue;
    if (parsed.pre.length === 0 || set.some(comparator => comparator.version.pre.length > 0 && comparator.version.major === parsed.major && comparator.version.minor === parsed.minor && comparator.version.patch === parsed.patch)) return true;
  }
  return false;
}

/** 🔢️ Orders two plain versions by semantic-version precedence; invalid input throws. */
export function semverOrderV1(left: string, right: string): -1 | 0 | 1 {
  const a = parseVersion(left), b = parseVersion(right);
  if (a === undefined || b === undefined) throw new TypeError(`Invalid semantic version: ${a === undefined ? left : right}`);
  return compareVersions(a, b);
}
