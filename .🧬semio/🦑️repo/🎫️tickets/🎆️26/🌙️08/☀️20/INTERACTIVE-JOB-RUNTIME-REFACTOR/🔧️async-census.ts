#!/usr/bin/env bun
// 🔧️async-census.ts
// TEMPORARY analysis tool for ticket INTERACTIVE-JOB-RUNTIME-REFACTOR, packet P0c.
// Lives only inside the ticket folder — NOT wired into 📜️script.ts, NOT a permanent script.
//
// For every `async fn` in every `.rs` file (outside compose/target/node_modules and a small
// set of clearly-non-source directories, see EXTRA_EXCLUDE below), this:
//   - locates the function and extracts its body by brace matching over a *cleaned* copy of the
//     file (line/block comments, string/byte-string/char literals and raw strings r#"…"#/br#"…"#
//     blanked out first, so braces/`.await`/keywords inside them can never be mis-counted)
//   - determines whether the body contains a genuine `.await` at ITS OWN level (awaits inside
//     nested closures / nested `async {}` blocks count; awaits inside a nested `fn` ITEM do not)
//   - classifies A / B / C / D per the ticket spec
//   - computes a transitive "A-shallow" refinement over a best-effort, name-keyed call graph
//
// Usage: bun 🔧️async-census.ts
// Outputs (written next to this script):
//   🔧️async-census.json   — one record per function
//   🔧️async-census-summary.json — aggregated counts used by the markdown report

import { readdirSync, statSync, readFileSync, writeFileSync } from "fs";
import { join, relative, sep } from "path";

//#region Config

const ROOT = "/Users/ueli/Documents/semio";
const OUT_DIR = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR";

// Directories excluded per explicit task instruction.
const MANDATORY_EXCLUDE = new Set(["compose", "target", "node_modules"]);

// Directories excluded because they are demonstrably NOT hand-written source subject to this
// refactor (ticket scratch snippets, build caches, generated/build output). Excluded from the
// CLASSIFIED census; their raw contribution is measured separately and reported for transparency.
const EXTRA_EXCLUDE = new Set([".🧬semio", "storybook-static", "♻️mit-bestand", ".nx", ".git", "⚡️cache"]);

// "large even without loop constructs" threshold for B/C split (task leaves this unspecified;
// documented explicitly here and in the report).
const LARGE_BODY_LINE_THRESHOLD = 80;

//#endregion

//#region Types

type Classification = "A" | "A-shallow" | "B" | "C" | "D";

interface FnRecord {
  file: string; // relative to ROOT
  area: string;
  plugin: string | null;
  name: string;
  line: number;
  pub: boolean;
  hasBody: boolean;
  classification: Classification;
  bodyLineCount: number;
  loopKeywords: string[];
  ownAwaitCount: number;
  awaitCalleeNames: string[];
  dReason: string | null;
}

//#endregion

//#region File walk

function shouldExclude(name: string): boolean {
  return MANDATORY_EXCLUDE.has(name) || EXTRA_EXCLUDE.has(name);
}

function walk(dir: string, out: string[]): void {
  let entries: string[];
  try {
    entries = readdirSync(dir);
  } catch {
    return;
  }
  for (const entry of entries) {
    if (shouldExclude(entry)) continue;
    const full = join(dir, entry);
    let st;
    try {
      st = statSync(full);
    } catch {
      continue;
    }
    if (st.isDirectory()) {
      walk(full, out);
    } else if (st.isFile() && entry.endsWith(".rs")) {
      out.push(full);
    }
  }
}

//#endregion

//#region Cleaning: blank out comments / string / char / raw-string content, preserve newlines

function isIdentChar(c: string | undefined): boolean {
  return !!c && /[A-Za-z0-9_]/.test(c);
}

export function cleanRustSource(src: string): string {
  const n = src.length;
  const out: string[] = new Array(n);
  let i = 0;
  while (i < n) {
    const c = src[i];

    // line comment
    if (c === "/" && src[i + 1] === "/") {
      while (i < n && src[i] !== "\n") {
        out[i] = " ";
        i++;
      }
      continue;
    }

    // block comment (nested)
    if (c === "/" && src[i + 1] === "*") {
      let depth = 1;
      out[i] = " ";
      out[i + 1] = " ";
      i += 2;
      while (i < n && depth > 0) {
        if (src[i] === "/" && src[i + 1] === "*") {
          depth++;
          out[i] = " ";
          out[i + 1] = " ";
          i += 2;
          continue;
        }
        if (src[i] === "*" && src[i + 1] === "/") {
          depth--;
          out[i] = " ";
          out[i + 1] = " ";
          i += 2;
          continue;
        }
        out[i] = src[i] === "\n" ? "\n" : " ";
        i++;
      }
      continue;
    }

    // raw string / raw byte string: (b)?r#*"..."#*
    if ((c === "r" || (c === "b" && src[i + 1] === "r")) && !isIdentChar(src[i - 1])) {
      let j = i;
      if (src[j] === "b") j++;
      if (src[j] === "r") {
        let k = j + 1;
        let hashes = 0;
        while (src[k] === "#") {
          hashes++;
          k++;
        }
        if (src[k] === '"') {
          for (let m = i; m <= k; m++) out[m] = src[m] === "\n" ? "\n" : " ";
          let p = k + 1;
          const closer = '"' + "#".repeat(hashes);
          let found = false;
          while (p < n) {
            if (src.startsWith(closer, p)) {
              for (let m = p; m < p + closer.length; m++) out[m] = src[m] === "\n" ? "\n" : " ";
              p += closer.length;
              found = true;
              break;
            }
            out[p] = src[p] === "\n" ? "\n" : " ";
            p++;
          }
          i = found ? p : n;
          continue;
        }
      }
    }

    // normal / byte string "..."
    if ((c === '"' || (c === "b" && src[i + 1] === '"')) && !isIdentChar(src[i - 1])) {
      let j = i;
      if (src[j] === "b") {
        out[j] = " ";
        j++;
      }
      out[j] = " ";
      let p = j + 1;
      while (p < n) {
        if (src[p] === "\\") {
          out[p] = " ";
          if (p + 1 < n) out[p + 1] = src[p + 1] === "\n" ? "\n" : " ";
          p += 2;
          continue;
        }
        if (src[p] === '"') {
          out[p] = " ";
          p++;
          break;
        }
        out[p] = src[p] === "\n" ? "\n" : " ";
        p++;
      }
      i = p;
      continue;
    }

    // char literal '...' (vs. lifetime 'a — only treated as literal on confirmed escape/short match)
    if (c === "'") {
      if (src[i + 1] === "\\") {
        let p = i + 2;
        if (src[p] === "u" && src[p + 1] === "{") {
          p += 2;
          while (p < n && src[p] !== "}") p++;
          p++;
        } else if (src[p] === "x") {
          p += 3;
        } else {
          p += 1;
        }
        if (src[p] === "'") {
          for (let m = i; m <= p; m++) out[m] = " ";
          i = p + 1;
          continue;
        }
      } else if (src[i + 1] !== undefined && src[i + 2] === "'") {
        out[i] = " ";
        out[i + 1] = " ";
        out[i + 2] = " ";
        i += 3;
        continue;
      }
      // else: lifetime / generic apostrophe — leave as-is, fall through
    }

    out[i] = src[i];
    i++;
  }
  return out.join("");
}

//#endregion

//#region Signature scanning (finds first top-level `{` or `;` after a name, tracking (), [], <>)

interface SigResult {
  hasBody: boolean;
  bodyStart?: number;
  declEnd?: number;
}

export function findBodyOrDecl(clean: string, from: number): SigResult | null {
  let paren = 0,
    brack = 0,
    angle = 0;
  let i = from;
  const n = clean.length;
  const limit = Math.min(n, from + 20000);
  while (i < limit) {
    const c = clean[i];
    if (c === "(") paren++;
    else if (c === ")") paren--;
    else if (c === "[") brack++;
    else if (c === "]") brack--;
    else if (c === "<") angle++;
    else if (c === ">") {
      if (clean[i - 1] !== "-" && angle > 0) angle--;
    } else if (paren <= 0 && brack <= 0 && angle <= 0) {
      if (c === "{") return { hasBody: true, bodyStart: i };
      if (c === ";") return { hasBody: false, declEnd: i };
    }
    i++;
  }
  return null;
}

//#endregion

//#region Body scan: brace-match, own-level `.await` count + callee names, loop keywords, nested-fn awareness

interface BodyScanResult {
  end: number;
  ownAwaitCount: number;
  awaitCalleeNames: string[];
  loopKeywords: Set<string>;
}

export function scanBody(clean: string, braceStart: number): BodyScanResult | null {
  const n = clean.length;
  // stack[k] === true  =>  scope at depth k+1 is inside a nested `fn` ITEM body
  const stack: boolean[] = [false];
  let ownAwaitCount = 0;
  const awaitCalleeNames: string[] = [];
  const loopKeywords = new Set<string>();
  let i = braceStart + 1;

  while (i < n) {
    const ch = clean[i];

    if (ch === "{") {
      stack.push(stack[stack.length - 1]);
      i++;
      continue;
    }
    if (ch === "}") {
      stack.pop();
      if (stack.length === 0) {
        return { end: i, ownAwaitCount, awaitCalleeNames, loopKeywords };
      }
      i++;
      continue;
    }

    if (/[A-Za-z_]/.test(ch)) {
      let j = i;
      while (j < n && /[A-Za-z0-9_]/.test(clean[j])) j++;
      const word = clean.slice(i, j);
      const nestedNow = stack[stack.length - 1];

      if (word === "fn") {
        let k = j;
        while (k < n && /\s/.test(clean[k])) k++;
        if (/[A-Za-z_]/.test(clean[k] ?? "")) {
          // nested named fn item — its own body (if any) does NOT count toward the outer fn
          let nameEnd = k;
          while (nameEnd < n && /[A-Za-z0-9_]/.test(clean[nameEnd])) nameEnd++;
          const sig = findBodyOrDecl(clean, nameEnd);
          if (sig && sig.hasBody && sig.bodyStart !== undefined) {
            i = sig.bodyStart;
            stack.push(true);
            i++;
            continue;
          } else if (sig && !sig.hasBody && sig.declEnd !== undefined) {
            i = sig.declEnd + 1;
            continue;
          }
        }
        i = j;
        continue;
      }

      if (!nestedNow && (word === "for" || word === "while" || word === "loop")) {
        loopKeywords.add(word);
      }
      if (!nestedNow && (word === "map" || word === "fold" || word === "for_each")) {
        const before = clean.slice(Math.max(0, i - 2), i);
        let k2 = j;
        while (k2 < n && /\s/.test(clean[k2])) k2++;
        if (before.trimEnd().endsWith(".") && clean[k2] === "(") {
          loopKeywords.add(word + "()");
        }
      }
      i = j;
      continue;
    }

    if (ch === "." && clean.slice(i, i + 6) === ".await") {
      const nestedNow = stack[stack.length - 1];
      if (!nestedNow) {
        ownAwaitCount++;
        awaitCalleeNames.push(extractCalleeName(clean, i));
      }
      i += 6;
      continue;
    }

    i++;
  }
  return null; // unbalanced — caller marks D
}

// best-effort: walk backward from a `.await` position to the name of the call it applies to.
// Documented limitation: purely lexical / name-keyed, cannot resolve overloads, trait dispatch,
// method-vs-function collisions, or std/external-crate callees.
function extractCalleeName(clean: string, awaitPos: number): string {
  let p = awaitPos - 1;
  while (p >= 0 && /\s/.test(clean[p])) p--;
  if (clean[p] === ")") {
    let depth = 1;
    p--;
    while (p >= 0 && depth > 0) {
      if (clean[p] === ")") depth++;
      else if (clean[p] === "(") depth--;
      p--;
    }
    let nameEnd = p + 1;
    let nameStart = nameEnd;
    while (nameStart > 0 && /[A-Za-z0-9_]/.test(clean[nameStart - 1])) nameStart--;
    const name = clean.slice(nameStart, nameEnd);
    return name.length > 0 ? name : "<unresolved:call-expr>";
  }
  // bare `.await` on a variable/future (e.g. `some_future.await`) — not a function name
  let nameEnd = p + 1;
  let nameStart = nameEnd;
  while (nameStart > 0 && /[A-Za-z0-9_]/.test(clean[nameStart - 1])) nameStart--;
  const name = clean.slice(nameStart, nameEnd);
  return name.length > 0 ? `<bare-future:${name}>` : "<unresolved:bare>";
}

//#endregion

//#region Area / plugin classification

function classifyArea(relPath: string): { area: string; plugin: string | null } {
  const parts = relPath.split(sep);
  if (parts[0] === "🧰️framework" && parts[1] === "🔨️modules") return { area: "framework/modules", plugin: null };
  if (parts[0] === "🧰️framework" && parts[1] === "🛍️products") return { area: "framework/products", plugin: null };
  if (parts[0] === "✏️s" && parts[1] === "🔌️plugins") return { area: "plugins", plugin: parts[2] ?? "<root>" };
  if (parts[0] === "✏️s" && parts[1] === "🔨️modules") return { area: "s/modules", plugin: null };
  return { area: "other", plugin: null };
}

//#endregion

//#region Main per-file processing

function processFile(absPath: string, records: FnRecord[]): void {
  const relPath = relative(ROOT, absPath);
  let src: string;
  try {
    src = readFileSync(absPath, "utf8");
  } catch {
    return;
  }
  const clean = cleanRustSource(src);
  const { area, plugin } = classifyArea(relPath);

  const lineStarts: number[] = [0];
  for (let i = 0; i < src.length; i++) if (src[i] === "\n") lineStarts.push(i + 1);
  const lineOf = (idx: number): number => {
    // binary search
    let lo = 0,
      hi = lineStarts.length - 1;
    while (lo < hi) {
      const mid = (lo + hi + 1) >> 1;
      if (lineStarts[mid] <= idx) lo = mid;
      else hi = mid - 1;
    }
    return lo + 1;
  };

  const fnRegex = /\basync\s+fn\s+([A-Za-z_][A-Za-z0-9_]*)/g;
  let m: RegExpExecArray | null;
  while ((m = fnRegex.exec(clean)) !== null) {
    const name = m[1];
    const matchStart = m.index;
    const nameEnd = matchStart + m[0].length;

    // pub detection: search back to line start for `pub` keyword
    const lineStart = src.lastIndexOf("\n", matchStart) + 1;
    const preamble = clean.slice(lineStart, matchStart);
    const isPub = /\bpub\b/.test(preamble);

    const sig = findBodyOrDecl(clean, nameEnd);
    const line = lineOf(matchStart);

    if (!sig) {
      records.push({
        file: relPath,
        area,
        plugin,
        name,
        line,
        pub: isPub,
        hasBody: false,
        classification: "D",
        bodyLineCount: 0,
        loopKeywords: [],
        ownAwaitCount: 0,
        awaitCalleeNames: [],
        dReason: "signature scan exceeded bound or hit EOF without `{`/`;`",
      });
      continue;
    }

    if (!sig.hasBody) {
      records.push({
        file: relPath,
        area,
        plugin,
        name,
        line,
        pub: isPub,
        hasBody: false,
        classification: "B",
        bodyLineCount: 0,
        loopKeywords: [],
        ownAwaitCount: 0,
        awaitCalleeNames: [],
        dReason: null,
      });
      continue;
    }

    const body = scanBody(clean, sig.bodyStart!);
    if (!body) {
      records.push({
        file: relPath,
        area,
        plugin,
        name,
        line,
        pub: isPub,
        hasBody: true,
        classification: "D",
        bodyLineCount: 0,
        loopKeywords: [],
        ownAwaitCount: 0,
        awaitCalleeNames: [],
        dReason: "brace matching did not close before EOF (unbalanced braces after cleaning)",
      });
      continue;
    }

    const bodyLineCount = src.slice(sig.bodyStart!, body.end).split("\n").length;
    const loopKeywords = Array.from(body.loopKeywords);
    let classification: Classification;
    if (body.ownAwaitCount > 0) {
      classification = "A";
    } else if (loopKeywords.length === 0 && bodyLineCount <= LARGE_BODY_LINE_THRESHOLD) {
      classification = "B";
    } else {
      classification = "C";
    }

    records.push({
      file: relPath,
      area,
      plugin,
      name,
      line,
      pub: isPub,
      hasBody: true,
      classification,
      bodyLineCount,
      loopKeywords,
      ownAwaitCount: body.ownAwaitCount,
      awaitCalleeNames: body.awaitCalleeNames,
      dReason: null,
    });

    // NOTE: deliberately do NOT skip fnRegex.lastIndex past `body.end` — a nested `async fn`
    // declared inside this body (local helper fn) must still get its own independent record.
    // scanBody() above already excludes such nested fn bodies from the OUTER function's own
    // await/loop-keyword accounting; the regex loop below will separately re-discover and
    // classify the nested fn on its own terms when it reaches that position.
  }
}

//#endregion

//#region Transitive A-shallow refinement

function refineAShallow(records: FnRecord[]): void {
  const nameClassMap = new Map<string, Set<Classification>>();
  for (const r of records) {
    if (!nameClassMap.has(r.name)) nameClassMap.set(r.name, new Set());
    nameClassMap.get(r.name)!.add(r.classification);
  }

  for (const r of records) {
    if (r.classification !== "A") continue;
    if (r.awaitCalleeNames.length === 0) continue;
    let allResolvedNonA = true;
    for (const calleeRaw of r.awaitCalleeNames) {
      if (calleeRaw.startsWith("<unresolved") || calleeRaw.startsWith("<bare-future")) {
        allResolvedNonA = false;
        break;
      }
      const classes = nameClassMap.get(calleeRaw);
      if (!classes || classes.has("A") || classes.has("A-shallow") || classes.has("D")) {
        allResolvedNonA = false;
        break;
      }
      // classes is a non-empty subset of {B, C} — safe
    }
    if (allResolvedNonA) r.classification = "A-shallow";
  }
}

//#endregion

//#region Run

// guarded so that importing this module (e.g. from 🔧️async-census-selftest.ts) does not
// trigger a full repo-wide census as a side effect of loading the parser primitives.
if (import.meta.main) {

const files: string[] = [];
walk(ROOT, files);

const records: FnRecord[] = [];
for (const f of files) processFile(f, records);
refineAShallow(records);

writeFileSync(join(OUT_DIR, "🔧️async-census.json"), JSON.stringify(records));

//#region Summary aggregation

function pct(n: number, total: number): string {
  return total === 0 ? "0.00" : ((n / total) * 100).toFixed(2);
}

const total = records.length;
const byClass: Record<string, number> = {};
for (const r of records) byClass[r.classification] = (byClass[r.classification] ?? 0) + 1;

const byArea: Record<string, Record<string, number>> = {};
for (const r of records) {
  byArea[r.area] ??= {};
  byArea[r.area][r.classification] = (byArea[r.area][r.classification] ?? 0) + 1;
}

const byPlugin: Record<string, Record<string, number>> = {};
for (const r of records) {
  if (r.area !== "plugins") continue;
  const key = r.plugin ?? "<root>";
  byPlugin[key] ??= {};
  byPlugin[key][r.classification] = (byPlugin[key][r.classification] ?? 0) + 1;
}

const dReasons: Record<string, number> = {};
for (const r of records) {
  if (r.classification === "D" && r.dReason) dReasons[r.dReason] = (dReasons[r.dReason] ?? 0) + 1;
}

const summary = {
  total,
  byClass,
  byClassPct: Object.fromEntries(Object.entries(byClass).map(([k, v]) => [k, pct(v, total)])),
  byArea,
  byPlugin,
  dReasons,
  filesScanned: files.length,
  largeBodyLineThreshold: LARGE_BODY_LINE_THRESHOLD,
};

writeFileSync(join(OUT_DIR, "🔧️async-census-summary.json"), JSON.stringify(summary, null, 2));

console.log(`files scanned: ${files.length}`);
console.log(`total async fn records: ${total}`);
console.log(JSON.stringify(byClass, null, 2));
console.log("sum check:", Object.values(byClass).reduce((a, b) => a + b, 0), "===", total);

} // import.meta.main

//#endregion
//#endregion
