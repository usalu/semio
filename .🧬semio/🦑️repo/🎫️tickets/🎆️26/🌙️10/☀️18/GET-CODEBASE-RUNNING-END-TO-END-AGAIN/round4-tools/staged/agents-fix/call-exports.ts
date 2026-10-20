const file = process.argv[2]!;
const module = await import(file);
for (const [name, value] of Object.entries(module)) {
  if (typeof value !== "function" || (value as Function).length !== 0 || !/^(verify|.*Oracle|.*SelfTests?|test.*|run.*)/.test(name)) continue;
  try { await (value as () => unknown)(); console.log("ok  ", name); } catch (error) { console.log("FAIL", name, String((error as Error)?.message ?? error).slice(0, 160)); }
}
