/** 🗂️ The case table of one mutation leaf: a JSON document `{ "<casesField>": ["<case>", …] }` committed as `<leaf>/🧪️tests/<table file>` that names every fixture bundle `🧪️tests/🧫️fixtures/🧬️mutations/<leaf>/<case>` its runners iterate. It is the language-neutral coverage source `mutationCatalogProjection.fixtureCoverage.leafCaseTable` declares; TypeScript, Rust and Python runners read the same bytes. Case names compare with U+FE0F removed and NFC applied. @see https://json-schema.org/understanding-json-schema/reference/array */
export interface MutationLeafCaseTablePolicy {
  readonly casesField: string;
}

export type MutationLeafCaseTable = Readonly<{ cases: readonly string[] }> | Readonly<{ error: string }>;

/** 🔤️ A case name as the coverage laws compare it: U+FE0F removed, NFC. */
export function foldMutationCaseName(name: string): string {
  return name.replaceAll("️", "").normalize("NFC");
}

/** 🧾️ Parses one case table source; the result is its case names in committed order or the reason it is not a case table. */
export function parseMutationLeafCaseTable(source: string, policy: MutationLeafCaseTablePolicy): MutationLeafCaseTable {
  let value: unknown;
  try {
    value = JSON.parse(source);
  } catch (error) {
    return { error: `is not JSON: ${(error as Error).message}` };
  }
  if (typeof value !== "object" || value === null || Array.isArray(value)) return { error: "must be a JSON object" };
  const keys = Object.keys(value);
  if (keys.length !== 1 || keys[0] !== policy.casesField) return { error: `must have exactly the member ${policy.casesField}` };
  const cases = (value as Record<string, unknown>)[policy.casesField];
  if (!Array.isArray(cases) || cases.length === 0) return { error: `${policy.casesField} must be a non-empty array` };
  const folded = new Set<string>();
  for (const item of cases) {
    if (typeof item !== "string" || item === "" || item !== item.trim() || /[\\/]/u.test(item)) return { error: `${policy.casesField} must hold plain directory names` };
    if (folded.has(foldMutationCaseName(item))) return { error: `${policy.casesField} lists ${item} twice` };
    folded.add(foldMutationCaseName(item));
  }
  return { cases: cases as string[] };
}

/** 🧮️ How a table disagrees with the fixture bundles of its leaf: a listed case without a bundle, or a bundle the table does not list. */
export function mutationLeafCaseTableProblems(listed: readonly string[], bundles: readonly string[]): readonly string[] {
  const listedFolds = new Set(listed.map(foldMutationCaseName)), bundleFolds = new Set(bundles.map(foldMutationCaseName));
  return [
    ...listed.filter((name) => !bundleFolds.has(foldMutationCaseName(name))).map((name) => `lists ${name}, which has no fixture bundle`),
    ...bundles.filter((name) => !listedFolds.has(foldMutationCaseName(name))).map((name) => `does not list the fixture bundle ${name}`),
  ];
}
