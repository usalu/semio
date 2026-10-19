const S = "/private/tmp/claude-501/-Users-ueli-Documents-semio/f251ca6a-3613-4a20-b3b7-6e7765237e44/scratchpad";
const lines = (await Bun.file(`${S}/ls-tree.txt`).text()).split("\n").filter(Boolean);
const entries = lines.map((l) => { const tab = l.indexOf("\t"); const [, type, sha] = l.slice(0, tab).split(" "); return { sha, type, path: l.slice(tab + 1) }; }).filter((e) => e.type === "blob");
const isTicket = (p: string) => p.startsWith(".🧬semio/");
const bySha = new Map<string, string[]>();
for (const e of entries) { const a = bySha.get(e.sha) ?? []; a.push(e.path); bySha.set(e.sha, a); }
const copies: string[] = [];
const srcExt = /\.(rs|ts|tsx|go|mjs|cjs|js|py|sh|ps1|toml|json|md|txt|feature|proto|graphql|sql)$/;
let ticketCopyOfTracked = 0, ticketCopyOfSource = 0;
for (const e of entries) {
  if (!isTicket(e.path)) continue;
  const others = (bySha.get(e.sha) ?? []).filter((p) => !isTicket(p));
  if (others.length) { ticketCopyOfTracked++; if (srcExt.test(e.path) && !/\.(md|txt|json)$/.test(e.path)) { ticketCopyOfSource++; copies.push(`${e.path}\t=> ${others[0]}`); } }
}
const orig = entries.filter((e) => !isTicket(e.path) && /\.orig$|\.DS_Store$|\.(log|bak|rej|swp)$/.test(e.path)).map((e) => e.path);
console.log({ blobs: entries.length, ticketFilesIdenticalToTrackedNonTicket: ticketCopyOfTracked, ticketCopiesOfCodeFiles: ticketCopyOfSource, staleTrackedJunk: orig.length });
console.log(orig.join("\n"));
await Bun.write(`${S}/ticket-copies.txt`, copies.join("\n") + "\n");
