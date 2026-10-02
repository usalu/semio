#!/usr/bin/env bun
/** 🏁️ Counts what `POST /commands` answers through Caddy for a body one byte over the limit, the way `checkBodyLimit`
 * of the deploy verbs sends it: `bun body_limit_race.ts <base url> <limit bytes> <attempts>`. Caddy and the proctor hold
 * the same limit, so either may refuse first; anything but `413` shows the race between them. */
const [base, limit, attempts] = [process.argv[2]!, Number(process.argv[3]), Number(process.argv[4])];
const answers: Record<string, number> = {};
for (let attempt = 0; attempt < attempts; attempt++) {
  const status = await fetch(`${base}/commands`, { method: "POST", headers: { origin: "https://quizze.architektur-und-technologie.de", "content-type": "application/json" }, body: new Uint8Array(limit + 1).fill(0x20), tls: { rejectUnauthorized: false } } as RequestInit).then((response) => String(response.status), (error: Error) => `error ${error.message}`);
  answers[status] = (answers[status] ?? 0) + 1;
}
console.log(JSON.stringify(answers));
