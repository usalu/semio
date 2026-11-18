S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R187"
t=open(S+"/base/p.ts").read()
def once(a,b):
    global t
    assert t.count(a)==1,(a[:80],t.count(a));t=t.replace(a,b)
once('type BilingualRefusalV1 = Readonly<{ name: "CargoLockRefused"; texts:','type BilingualRefusalV1 = Readonly<{ name: "CargoLockRefused" | "WindowsRenameRefused"; texts:')
once('''  return typeof value === "object" && row !== null && row.name === "CargoLockRefused" && typeof row.texts === "object" && row.texts !== null;
};
''','''  return typeof value === "object" && row !== null && (row.name === "CargoLockRefused" || row.name === "WindowsRenameRefused") && typeof row.texts === "object" && row.texts !== null;
};

/** 🔎️ The first typed bilingual refusal of a fault, looked for through aggregated faults and causes (first failure first, bounded depth), so a settlement that wrapped it renders it instead of an internal error. */
const firstBilingualRefusalV1 = (error: unknown, depth = 0): BilingualRefusalV1 | undefined => {
  if (isBilingualRefusalV1(error)) return error;
  if (depth >= 8 || !(error instanceof Error)) return undefined;
  for (const member of error instanceof AggregateError ? error.errors : error.cause === undefined ? [] : [error.cause]) {
    const found = firstBilingualRefusalV1(member, depth + 1);
    if (found) return found;
  }
  return undefined;
};
''')
once('  const bilingual = isBilingualRefusalV1(error) ? error : undefined;','  const bilingual = firstBilingualRefusalV1(error);')
open(S+"/stage/p.ts","w").write(t)
print("ok")
