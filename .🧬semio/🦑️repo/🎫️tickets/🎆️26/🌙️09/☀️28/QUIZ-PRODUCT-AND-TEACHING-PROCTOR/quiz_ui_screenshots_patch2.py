import io, sys
p = sys.argv[1]
s = io.open(p, encoding="utf-8", newline="").read()
reps = [
('    page.on("pageerror", (error) => homeErrors.push(error.message));', '    page.on("pageerror", (error) => homeErrors.push(`pageerror: ${error.stack ?? error.message}`));'),
('    page.on("console", (message) => message.type() === "error" && !message.text().includes("ERR_FAILED") && homeErrors.push(message.text()));', '    page.on("console", (message) => message.type() === "error" && !message.text().includes("ERR_FAILED") && homeErrors.push(`console: ${message.text()} @ ${JSON.stringify(message.location())}`));'),
('  for (const scheme of schemes) {', '  if (process.env.ONLY_WIDTH !== undefined && String(width) !== process.env.ONLY_WIDTH) continue;\n  for (const scheme of schemes) {'),
]
for a, b in reps:
    if s.count(a) != 1:
        sys.exit(f"anchor {s.count(a)}: {a[:60]!r}")
    s = s.replace(a, b)
io.open(p, "w", encoding="utf-8", newline="").write(s)
print("patched")
