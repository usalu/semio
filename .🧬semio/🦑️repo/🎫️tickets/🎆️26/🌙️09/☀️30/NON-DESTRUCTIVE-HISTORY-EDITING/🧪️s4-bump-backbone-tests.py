#!/usr/bin/env python3
"""🧪️ S4-BUMP: re-seals the TS channel-twin suite (`💻️os/🧪️tests/🧪️backbone-envelope-io/🟦️.ts`) for CHANNEL_VERSION 21.

Deletes every `LoadDocument` literal/golden and `AppChannelClient.loadDocument` caller, drops `transactionPrepare.label` and
`transactionProposal.{description, coalesce_key}`, pins tag 6 as unassigned, and moves the document-cache acceptance law
(same fixture, same schema) onto `loadDocumentArchive`: the candidate is the archive's root pair, cached only on `ready`.
Every anchor must match exactly as often as stated.
"""
import pathlib
import re
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
SUITE = ROOT / "🧰️framework/🛍️products/💻️os/🧪️tests/🧪️backbone-envelope-io/🟦️.ts"
OLD_LAW = ROOT / ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🗑️generated/s4-bump/old-document-cache-law.txt"

NEW_LAW = '''    it("publishes only accepted document cache candidates and owns both byte arrays", async () => {
      const { readFileSync } = await import("node:fs");
      const fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/📦️document-cache/🔣️.json", source.url), "utf8"));
      const schema = JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json", source.url), "utf8"));
      const validate = semioSchemaAjvV1({ strict: true }).addSchema(schema).getSchema(`${schema.$id}#/$defs/DocumentCacheAcceptanceV1`)!;
      expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
      expect(validate({ ...fixture, optimistic: true })).toBe(false);
      const pair = (value: { pack: number[]; spr: number[] }) => ({ pack: Uint8Array.from(value.pack), spr: Uint8Array.from(value.spr) });
      const archiveOf = (value: { pack: number[]; spr: number[] }) => ({ parent_pack: [...value.pack], parent_spr: [...value.spr], members: [] });
      for (const row of fixture.cases) {
        const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
        let admission = "done";
        const client = new AppChannelClient({
          outcomes: broadcast.stream,
          enqueue: (_instanceId: number, commands: readonly Uint8Array[]) => {
            const command = decodeAppCommand(commands[0]!);
            if ("LoadDocumentArchive" in command && admission === "enqueue") throw new Error("document-cache.enqueue");
            const seq = commandSeq(command);
            const document = { Document: { in_reply_to: seq, pack: fixture.reply.pack, spr: fixture.reply.spr, ops: "" } };
            const error = { Error: { in_reply_to: seq, fault: [99], report: [] } };
            const reply: AppFrameValue[] | "transport" =
              "PollDocumentArchiveLoad" in command ? [{ DocumentArchiveLoad: { in_reply_to: seq, status: { operation: command.PollDocumentArchiveLoad.operation, state: "ready", completed: 1, total: 1, fault: [] } } }] :
              !("LoadDocumentArchive" in command) || admission === "done" ? [{ Done: { in_reply_to: seq } }] :
              admission === "transport" ? "transport" :
              admission === "document" ? [document] :
              admission === "error-document" ? [document, error] : [error];
            queueMicrotask(() => broadcast.push(reply === "transport" ? { instanceId: 1, error: new Error("document-cache.transport") } : { instanceId: 1, frames: reply.map(encodeAppFrame) }));
          },
        }, new AppChannelRequestSequence(), 1, "cache");
        try {
          await client.loadDocumentArchive(archiveOf(fixture.initial));
          expect(client.documentPack()).toEqual(pair(fixture.initial));
          admission = row.outcome;
          const candidate = archiveOf(fixture.candidate);
          const settled = client.loadDocumentArchive(candidate).then(() => "resolved", () => "rejected");
          candidate.parent_pack.fill(255);
          candidate.parent_spr.fill(255);
          expect(client.documentPack()).toEqual(pair(fixture.initial));
          expect(await settled).toBe(row.outcome === "done" ? "resolved" : "rejected");
          expect(client.documentPack()).toEqual(pair(fixture[row.expected]));
          const exposed = client.documentPack()!;
          exposed.pack.fill(254);
          exposed.spr.fill(254);
          expect(client.documentPack()).toEqual(pair(fixture[row.expected]));
          client.dispose();
          expect(client.documentPack()).toBeNull();
        } finally {
          client.dispose();
          broadcast.complete();
        }
      }
    });

'''


def replace(text: str, old: str, new: str, count: int = 1) -> str:
    found = text.count(old)
    if found != count:
        sys.exit(f"anchor matched {found}x (expected {count}): {old[:140]!r}")
    return text.replace(old, new)


def sub(text: str, pattern: str, new: str, count: int) -> str:
    result, found = re.subn(pattern, new, text)
    if found != count:
        sys.exit(f"pattern matched {found}x (expected {count}): {pattern!r}")
    return result


text = SUITE.read_text()
old_law = OLD_LAW.read_text()
old_law_and_echo = old_law
text = replace(text, old_law_and_echo, NEW_LAW)
text = replace(text, "      { LoadDocument: { seq: 8, pack: [1, 2, 3], spr: [4, 5, 6] } },\n", "")
text = sub(text, r'(prepared_ops: [^\n]*?\], )label: "[^"\n]*", (origin: )', r"\1\2", 9)
text = sub(text, r'description: "[^"\n]*", coalesce_key: "[^"\n]*", ', "", 3)
text = replace(
    text,
    "      expect(encodeAppCommand({ AcknowledgeDocumentArchiveLoad: { seq: 0, operation: 1 } })[0]).toBe(36);\n    });\n",
    "      expect(encodeAppCommand({ AcknowledgeDocumentArchiveLoad: { seq: 0, operation: 1 } })[0]).toBe(36);\n"
    "      expect(() => decodeAppCommand(new Uint8Array([6, 8, 1, 1, 1, 2]))).toThrow(\"decodeAppCommand: unknown tag 6\");\n    });\n",
)
text = replace(text, '        ["LoadDocument", { LoadDocument: { seq: 1, pack: [1], spr: [2] } }],\n', "")
text = replace(text, '        LoadDocument: "060101010102",\n', "")
text = replace(
    text,
    '''    it("configure()/readDocument()/loadDocument() frame the right AppCommand variant", async () => {''',
    '''    it("configure()/readDocument() frame the right AppCommand variant", async () => {''',
)
text = replace(
    text,
    '''      await client.readDocument();
      await client.loadDocument(new Uint8Array([1]), new Uint8Array([2]));
      expect(seen[0]).toEqual({ ConfigCommand: { seq: 1, command: Array.from(encodePackValue({ locale: "en" })) } });
      expect(seen[1]).toEqual({ ReadDocument: { seq: 2 } });
      expect(seen[2]).toEqual({ LoadDocument: { seq: 3, pack: [1], spr: [2] } });
''',
    '''      await client.readDocument();
      expect(seen).toEqual([{ ConfigCommand: { seq: 1, command: Array.from(encodePackValue({ locale: "en" })) } }, { ReadDocument: { seq: 2 } }]);
''',
)
text = replace(text, '[new Uint8Array([2]), new Uint8Array([3])], "duplicate", new Uint8Array([4]));', "[new Uint8Array([2]), new Uint8Array([3])], new Uint8Array([4]));")
SUITE.write_text(text)
print("backbone-envelope-io re-sealed")
