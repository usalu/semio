import io, sys
p = sys.argv[1]
s = io.open(p, encoding="utf-8", newline="").read()
reps = [
("""      headings: [...document.querySelectorAll("h1, h2")].map((heading) => `${heading.tagName}:${heading.textContent}`),
    };""",
"""      headings: [...document.querySelectorAll("h1, h2")].map((heading) => `${heading.tagName}:${heading.textContent}`),
      wrapped: [...document.querySelectorAll("table th[scope=col], table .quiz-nowrap")].flatMap((cell) => {
        const range = document.createRange();
        range.selectNodeContents(cell);
        const lines = new Set([...range.getClientRects()].filter((rect) => rect.width > 0).map((rect) => Math.round(rect.top)));
        return lines.size > 1 ? [`${cell.textContent?.trim()} (${lines.size} lines)`] : [];
      }),
      nowrapHeads: [...document.querySelectorAll("table th[scope=col]")].every((cell) => getComputedStyle(cell).whiteSpace === "nowrap"),
    };"""),
("""  readonly headings: readonly string[];
}""",
"""  readonly headings: readonly string[];
  readonly wrapped: readonly string[];
  readonly nowrapHeads: boolean;
}"""),
("""for (const [width, height] of [
  [1440, 900],
  [768, 1024],
  [375, 812],
] as const) {
  for (const scheme of ["light", "dark"] as const) {""",
"""for (const [width, height, schemes] of [
  [1440, 900, ["light", "dark"]],
  [1280, 800, ["light"]],
  [1024, 768, ["light"]],
  [768, 1024, ["light", "dark"]],
  [375, 812, ["light", "dark"]],
] as const) {
  for (const scheme of schemes) {"""),
("""for (const report of reports) console.log(`[DEBUG] ${report.name} ${report.width} ${report.scheme} columns=${report.columns ?? "-"} overflow=${report.overflow} errors=${report.errors.length}`);""",
"""for (const report of reports) console.log(`[DEBUG] ${report.name} ${report.width} ${report.scheme} columns=${report.columns ?? "-"} overflow=${report.overflow} errors=${report.errors.length} wrapped=${report.wrapped.length} nowrapHeads=${report.nowrapHeads}`);"""),
]
for a, b in reps:
    if s.count(a) != 1:
        sys.exit(f"anchor {s.count(a)}: {a[:60]!r}")
    s = s.replace(a, b)
io.open(p, "w", encoding="utf-8", newline="").write(s)
print("patched")
