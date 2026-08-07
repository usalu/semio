const mod = await import("../../../../../../\ud83d\udcdc\ufe0fscript.ts");
const lint = mod.policy;
console.log("typeof", typeof lint);
console.log("keys", lint && Object.keys(lint));
const runner = lint?.check || lint?.run || lint?.lint || (typeof lint === "function" ? lint : null);
if (!runner) {
  console.log("no runner", lint);
  process.exit(2);
}
const breaches = await runner({});
const kinds = {};
for (const b of breaches) kinds[b.kind] = (kinds[b.kind] || 0) + 1;
const focus = breaches.filter((b) =>
  /plugin-root|plugin-builder|banned|emoji-prefix|name-stem|plugin\/|BannedName|EmojiPrefix/i.test(
    `${b.kind} ${b.id} ${b.summary}`,
  ),
);
const byPri = {};
for (const b of focus) byPri[b.priority] = (byPri[b.priority] || 0) + 1;
console.log(JSON.stringify({ total: breaches.length, kinds, focus: focus.length, byPri, sample: focus.slice(0, 40) }, null, 2));
