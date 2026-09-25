/** 🔌 Hub Protocol v1 websocket e2e client (scratch, ticket folder): two sockets (alice owner, bob editor) observe presence + operation broadcasts and compare hashes. Usage: `bun hub-e2e-ws.ts <base> <sessionId> <aliceToken> <bobToken>`. */
const [base, sessionId, alice, bob] = process.argv.slice(2);
const wsBase = base.replace(/^http/, "ws");
type Message = Record<string, any>;

function open(name: string, token: string, clientId: string) {
  const inbox: Message[] = [];
  const waiters: Array<() => void> = [];
  const ws = new WebSocket(`${wsBase}/sessions/${sessionId}/ws?token=${token}&clientId=${clientId}&client=e2e`);
  ws.onmessage = (event) => {
    const message = JSON.parse(String(event.data));
    console.log(`[ws ${name}] <- ${JSON.stringify(message).slice(0, 400)}`);
    inbox.push(message);
    waiters.splice(0).forEach((wake) => wake());
  };
  ws.onclose = () => console.log(`[ws ${name}] closed`);
  const until = async (match: (m: Message) => boolean, timeoutMs = 20000): Promise<Message> => {
    const deadline = Date.now() + timeoutMs;
    for (;;) {
      const index = inbox.findIndex(match);
      if (index >= 0) return inbox.splice(index, 1)[0];
      if (Date.now() > deadline) throw new Error(`[ws ${name}] timeout`);
      await new Promise<void>((resolve) => { waiters.push(resolve); setTimeout(resolve, 200); });
    }
  };
  const send = (message: Message) => {
    console.log(`[ws ${name}] -> ${JSON.stringify(message).slice(0, 400)}`);
    ws.send(JSON.stringify(message));
  };
  return { ws, until, send };
}

const createDesign = "mutation($storeId: ID!, $changeId: ID!, $id: ID!, $name: String!) { session { store(id: $storeId) { theKit { unsavedChange(id: $changeId) { kit { createDesign(id: $id, name: $name) { ok errors { message } } } } } } } }";
const renameKit = "mutation($storeId: ID!, $changeId: ID!, $name: String!) { session { store(id: $storeId) { theKit { unsavedChange(id: $changeId) { kit { rename(newName: $name) { ok errors { message } } } } } } } }";

const a = open("alice", alice, "alice-e2e");
const welcomeA = await a.until((m) => m.type === "welcome");
const b = open("bob", bob, "bob-e2e");
const welcomeB = await b.until((m) => m.type === "welcome");
await a.until((m) => m.type === "presence.joined" && m.participant.id === welcomeB.self.id);
console.log(`[e2e] welcome alice v${welcomeA.version} ${welcomeA.hash} participants=${welcomeA.participants.length}; welcome bob participants=${welcomeB.participants.length}`);

b.send({ type: "presence", cursor: { x: 12.5, y: -3, space: "design" }, focus: { app: "sketchpad" } });
const updated = await a.until((m) => m.type === "presence.updated");
console.log(`[e2e] alice saw bob presence cursor=${JSON.stringify(updated.participant.cursor)} focus=${JSON.stringify(updated.participant.focus)}`);

const designId = Bun.randomUUIDv7();
b.send({ type: "operation", operationId: `bob-${designId}`, baseVersion: welcomeB.version, query: createDesign, variables: { id: designId, name: "E2E Capsule Cluster" } });
const opA1 = await a.until((m) => m.type === "operation");
const opB1 = await b.until((m) => m.type === "operation");
console.log(`[e2e] createDesign broadcast v${opA1.version}: alice hash ${opA1.hash} / bob hash ${opB1.hash} (participant ${opA1.participantId === welcomeB.self.id ? "bob" : "?"})`);

const posted = await fetch(`${base}/sessions/${sessionId}/operations`, {
  method: "POST",
  headers: { authorization: `Bearer ${alice}`, "content-type": "application/json" },
  body: JSON.stringify({ operationId: `alice-${Bun.randomUUIDv7()}`, clientId: "alice-e2e", baseVersion: opA1.version, query: renameKit, variables: { name: "Metabolism E2E Remix" } }),
}).then((r) => r.json());
console.log(`[e2e] alice POST rename -> ${JSON.stringify(posted).slice(0, 300)}`);
const opA2 = await a.until((m) => m.type === "operation");
const opB2 = await b.until((m) => m.type === "operation");
console.log(`[e2e] rename broadcast v${opA2.version}: alice hash ${opA2.hash} / bob hash ${opB2.hash} (participant ${opA2.participantId === welcomeA.self.id ? "alice" : "?"})`);

const kit = await fetch(`${base}/sessions/${sessionId}/kit`, { headers: { authorization: `Bearer ${bob}` } }).then((r) => r.json());
const designs = (kit.kit.typologies?.items ?? []).flatMap((t: any) => t.designs?.items ?? []).map((d: any) => d.id);
console.log(`[e2e] GET kit v${kit.version} hash ${kit.hash} name=${kit.kit.name} containsDesign=${designs.includes(designId)}`);
const consistent = opA1.hash === opB1.hash && opA2.hash === opB2.hash && opA2.hash === posted.hash && kit.hash === opA2.hash && kit.version === opA2.version;
console.log(`[e2e] hash consistency: ${consistent ? "OK" : "MISMATCH"}`);

a.send({ type: "ping" });
await a.until((m) => m.type === "pong");
b.ws.close();
const left = await a.until((m) => m.type === "presence.left");
console.log(`[e2e] alice saw presence.left ${left.participantId === welcomeB.self.id ? "bob" : left.participantId}`);
a.ws.close();
await new Promise((resolve) => setTimeout(resolve, 300));
process.exit(consistent && designs.includes(designId) ? 0 : 1);
