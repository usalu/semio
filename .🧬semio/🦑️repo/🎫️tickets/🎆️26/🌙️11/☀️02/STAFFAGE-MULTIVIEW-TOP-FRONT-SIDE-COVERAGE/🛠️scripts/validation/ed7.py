import sys

p = sys.argv[1]
s = open(p, encoding="utf8").read()


def rep(a, b):
    global s
    assert a in s, a[:70]
    s = s.replace(a, b, 1)


rep(
    "readonly worst?: { readonly deltaMm: number; readonly subject: string; readonly pair: string } };",
    "readonly worst?: { readonly deltaMm: number; readonly subject: string; readonly pair: string }; readonly byKey?: Readonly<Record<string, { readonly deltaMm: number; readonly subject: string }>> };",
)
rep(
    "worst?: { deltaMm: number; subject: string; pair: string } }> }>();",
    "worst?: { deltaMm: number; subject: string; pair: string }; byKey?: Record<string, { deltaMm: number; subject: string }> }> }>();",
)
rep(
    "        if (entry.first.length < 3) entry.first.push({ subject, message });",
    """        if (delta !== undefined && mode === "light-line") {
          const byKey = (entry.byKey ??= {});
          const known = byKey[delta.pair];
          if (known === undefined || delta.deltaMm > known.deltaMm) byKey[delta.pair] = { deltaMm: delta.deltaMm, subject };
        }
        if (entry.first.length < 3) entry.first.push({ subject, message });""",
)
open(p, "w", encoding="utf8").write(s)
