"""🔌️ C12 session 14b: ports the ticket-local `probe-c12-shortage.mjs` into the os-dev collab-e2e harness (preamble rule 17) as
STEP 15 — real link cuts (TCP relays between each shell and the hub) of 5 s / 15 s / 60 s while user1 types. One-off, idempotent."""
import sys

path = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🤝️collaboration/🟦️.ts"
text = open(path, encoding="utf-8").read()
if "collabStartLinkRelay" in text:
    print("already applied")
    sys.exit(0)


def sub(old: str, new: str) -> None:
    global text
    assert text.count(old) == 1, (text.count(old), old[:100])
    text = text.replace(old, new)


sub('import { spawnSync } from "node:child_process";\n', 'import { spawnSync } from "node:child_process";\n\nimport { connect, createServer, type AddressInfo, type Socket } from "node:net";\n')

sub('''  "writer/draw/puzzle3d surfaces show peer-cursor overlay markers that move when the peer pointer moves",
] as const;
''', '''  "writer/draw/puzzle3d surfaces show peer-cursor overlay markers that move when the peer pointer moves",
  "real link cuts (5 s / 15 s / 60 s) while user1 types: no shell freezes, nothing is rebuilt, every keystroke typed during a cut reaches the hub and user2",
] as const;

/** ⏱️ STEP 15's link cuts in milliseconds (`S_COLLAB_LINK_CUTS_MS`, comma-separated): a 5 s blip, a 15 s shortage and a 60 s one at
 * the link-shortage policy's bound. */
const COLLAB_E2E_LINK_CUTS_MS = (process.env.S_COLLAB_LINK_CUTS_MS ?? "5000,15000,60000").split(",").map(Number);

/** 🔌️ A TCP relay between one shell and the hub that STEP 15 cuts for real: every relayed connection is destroyed and new ones are
 * refused until the cut ends (`setOffline` leaves an open WebSocket silently black-holed instead). Transparent otherwise, so every
 * other step runs through it unchanged. */
type CollabLinkRelay = { readonly url: string; cut(ms: number): void; close(): Promise<void> };

async function collabStartLinkRelay(hubBaseUrl: string): Promise<CollabLinkRelay> {
  const upstream = new URL(hubBaseUrl);
  const pairs = new Set<readonly [Socket, Socket]>();
  let cutUntil = 0;
  const server = createServer((client) => {
    if (Date.now() < cutUntil) {
      client.destroy();
      return;
    }
    const hub = connect(Number(upstream.port), upstream.hostname);
    const pair = [client, hub] as const;
    pairs.add(pair);
    const drop = (): void => {
      pairs.delete(pair);
      client.destroy();
      hub.destroy();
    };
    for (const socket of pair) {
      socket.on("error", drop);
      socket.on("close", drop);
    }
    client.pipe(hub);
    hub.pipe(client);
  });
  await new Promise<void>((resolve) => server.listen(0, "127.0.0.1", resolve));
  const cutAll = (): void => {
    for (const [client, hub] of pairs) {
      client.destroy();
      hub.destroy();
    }
    pairs.clear();
  };
  return {
    url: `http://127.0.0.1:${(server.address() as AddressInfo).port}`,
    cut: (ms) => {
      cutUntil = Date.now() + ms;
      cutAll();
    },
    close: () =>
      new Promise<void>((resolve) => {
        cutAll();
        server.close(() => resolve());
      }),
  };
}

/** 🎞️ One animation frame's latency on `page`, in ms: a frozen shell answers late or not at all. */
async function collabFrameMs(page: import("playwright").Page): Promise<number> {
  return Math.round(await page.evaluate(() => new Promise<number>((resolve) => { const started = performance.now(); requestAnimationFrame(() => resolve(performance.now() - started)); })));
}

/** 🔁️ Whether `page` shows the rebuild notice ("fresh authoritative restore", en or de): the document was discarded and rebuilt. */
async function collabShowsRebuild(page: import("playwright").Page): Promise<boolean> {
  return (await page.getByText(/fresh authoritative restore|autoritative Wiederherstellung/u).count()) > 0;
}
''')

sub('''  readonly spaceId: string | undefined;
  readonly artifactId: string | undefined;
}): Promise<void> {
  if (!opts.spaceId || !opts.artifactId) {
    for (const step of [11, 12, 13, 14]) opts.record(step, false, "skipped — no space/artifact id from earlier steps");
    return;
  }''', '''  readonly spaceId: string | undefined;
  readonly artifactId: string | undefined;
  readonly relays: readonly [CollabLinkRelay, CollabLinkRelay];
}): Promise<void> {
  if (!opts.spaceId || !opts.artifactId) {
    for (const step of [11, 12, 13, 14, 15]) opts.record(step, false, "skipped — no space/artifact id from earlier steps");
    return;
  }''')

sub('''    opts.record(14, false, error instanceof Error ? error.message : String(error));
  }
}
''', '''    opts.record(14, false, error instanceof Error ? error.message : String(error));
  }

  try {
    const rounds: string[] = [];
    for (const outageMs of COLLAB_E2E_LINK_CUTS_MS) {
      const settled = await collabAwaitConvergence(opts.user1, editor1, editor2, 30_000);
      spaceE2eAssert(settled.converged, `the editors disagreed before the ${outageMs} ms cut`);
      const noticeBefore = (await collabShowsRebuild(opts.user1)) || (await collabShowsRebuild(opts.user2));
      const cutAt = Date.now();
      for (const relay of opts.relays) relay.cut(outageMs);
      const markers: string[] = [];
      let worstFrameMs = 0;
      let rebuilt = false;
      while (Date.now() < cutAt + Math.min(outageMs - 1_000, 20_000)) {
        const marker = `cut${outageMs / 1_000}k${markers.length}`;
        await editor1.focus();
        await opts.user1.keyboard.press("End");
        await opts.user1.keyboard.type(` ${marker}`, { delay: 35 });
        markers.push(marker);
        worstFrameMs = Math.max(worstFrameMs, await collabFrameMs(opts.user1), await collabFrameMs(opts.user2));
        rebuilt ||= (await collabShowsRebuild(opts.user1)) || (await collabShowsRebuild(opts.user2));
      }
      const remaining = cutAt + outageMs - Date.now();
      if (remaining > 0) await opts.user1.waitForTimeout(remaining);
      const restoredAt = Date.now();
      let recovered = await collabAwaitConvergence(opts.user1, editor1, editor2, 120_000);
      while (Date.now() - restoredAt < 120_000 && !markers.every((marker) => recovered.first.includes(marker) && recovered.second.includes(marker))) {
        await opts.user1.waitForTimeout(500);
        recovered = await collabAwaitConvergence(opts.user1, editor1, editor2, 10_000);
      }
      rebuilt ||= (await collabShowsRebuild(opts.user1)) || (await collabShowsRebuild(opts.user2));
      const missing = markers.filter((marker) => !recovered.first.includes(marker) || !recovered.second.includes(marker));
      spaceE2eAssert(worstFrameMs < 1_000, `a shell froze during the ${outageMs} ms cut (worst frame ${worstFrameMs} ms)`);
      spaceE2eAssert(noticeBefore || !rebuilt, `the ${outageMs} ms cut rebuilt the document instead of resuming it`);
      spaceE2eAssert(
        recovered.converged && missing.length === 0,
        `keystrokes typed during the ${outageMs} ms cut were lost: ${JSON.stringify(missing)} (user1: ${JSON.stringify(recovered.first.slice(-160))}, user2: ${JSON.stringify(recovered.second.slice(-160))})`,
      );
      rounds.push(`${outageMs / 1_000} s: ${markers.length} markers in both shells ${Date.now() - restoredAt} ms after the link returned, worst frame ${worstFrameMs} ms`);
    }
    opts.record(15, true, rounds.join("; "));
  } catch (error) {
    await collabScreenshot(opts.user1, "step15-user1");
    await collabScreenshot(opts.user2, "step15-user2");
    opts.record(15, false, error instanceof Error ? error.message : String(error));
  }
}
''')

sub('''  let browser: import("playwright").Browser | undefined;
  const results: CollabStepOutcome[] = [];
  const record = collabRecorder(results);

  const teardown = async (): Promise<void> => {
    try {
      await browser?.close();
    } catch {
      // 🏁️ Best-effort.
    }
''', '''  let browser: import("playwright").Browser | undefined;
  let relays: readonly [CollabLinkRelay, CollabLinkRelay] | undefined;
  const results: CollabStepOutcome[] = [];
  const record = collabRecorder(results);

  const teardown = async (): Promise<void> => {
    try {
      await browser?.close();
    } catch {
      // 🏁️ Best-effort.
    }
    for (const relay of relays ?? []) await relay.close();
''')

sub('''    try {
      [user1Daemon, user2Daemon] = await Promise.all([
        collabStartUserDevServer({ port: user1Port, hubUrl: hubBaseUrl, user: COLLAB_E2E_USER1_EMAIL, dataDir: user1DataDir, logPath: join(outDir, "🧪️3-c-user1-dev.txt") }),
        collabStartUserDevServer({ port: user2Port, hubUrl: hubBaseUrl, user: COLLAB_E2E_USER2_EMAIL, dataDir: user2DataDir, logPath: join(outDir, "🧪️3-c-user2-dev.txt") }),
      ]);''', '''    try {
      relays = [await collabStartLinkRelay(hubBaseUrl), await collabStartLinkRelay(hubBaseUrl)];
      [user1Daemon, user2Daemon] = await Promise.all([
        collabStartUserDevServer({ port: user1Port, hubUrl: relays[0].url, user: COLLAB_E2E_USER1_EMAIL, dataDir: user1DataDir, logPath: join(outDir, "🧪️3-c-user1-dev.txt") }),
        collabStartUserDevServer({ port: user2Port, hubUrl: relays[1].url, user: COLLAB_E2E_USER2_EMAIL, dataDir: user2DataDir, logPath: join(outDir, "🧪️3-c-user2-dev.txt") }),
      ]);''')

sub('''    await collabRunCollaborationBehaviours({ record, user1: user1Page, user2: user2Page, spaceId: scenario.spaceId, artifactId: scenario.artifactId });''', '''    await collabRunCollaborationBehaviours({ record, user1: user1Page, user2: user2Page, spaceId: scenario.spaceId, artifactId: scenario.artifactId, relays: relays! });''')

open(path, "w", encoding="utf-8").write(text)
print("applied")
