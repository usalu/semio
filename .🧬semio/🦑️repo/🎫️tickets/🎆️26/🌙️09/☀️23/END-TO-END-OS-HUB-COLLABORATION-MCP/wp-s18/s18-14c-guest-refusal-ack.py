# -*- coding: utf-8 -*-
"""S18 §14c (S20 relay): one guest refusal must never poison a hub document's dispatch owner. The store worker's browser
actor threw `action-guest-refused` the moment it met a typed-operation FAULT page — before acknowledging it and before
routing the rest of the turn — so the guest kept that operation parked on an unacknowledged page, the next action came
back `action-state-unconfirmed`, the actor closed, and every later action was `action-owner-mismatch`. Now every page of
an action turn is answered (`typedOperationPageAnswerV1`: a fault page too), the turn is driven to its end, and the
action is refused explicitly afterwards (the actor stays open). Idempotent."""
import pathlib

WIRE = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts")
LAWS = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🧪️tests/🗞️typed-operation-page/🟦️.ts")
WORKER = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts")

EDITS = [
    (WIRE,
     "/** 🔁️ The bounded poll that keeps a mounted typed operation advancing once no host call is left to\n",
     """/** 📬️ What ONE typed-operation result page asks of the host driving an action turn: its acknowledgement — a fault
 * page's too, since the guest parks the operation until every page lands and an unanswered fault page left the
 * document's actor unable to take its next action — whether it is the guest's refusal of the action, and whether it
 * rides the artifact lane. Policy stays with the caller: it refuses the action only after the whole turn is answered. */
export function typedOperationPageAnswerV1(page: TypedOperationPage): Readonly<{ acknowledgement: TypedOperationAckEvent; refused: boolean; artifact: boolean }> {
  return { acknowledgement: page.acknowledgement, refused: page.lane === TYPED_OPERATION_LANE_FAULT, artifact: page.lane === TYPED_OPERATION_LANE_ARTIFACT };
}

/** 🔁️ The bounded poll that keeps a mounted typed operation advancing once no host call is left to
"""),
    (WIRE,
     "      typedOperationAcknowledgements,\n      typedOperationResult,\n      TYPED_OPERATION_ACK_MAGIC,\n",
     "      typedOperationAcknowledgements,\n      typedOperationPageAnswerV1,\n      typedOperationResult,\n      TYPED_OPERATION_ACK_MAGIC,\n"),
    (LAWS,
     "    typedOperationAcknowledgements,\n    typedOperationResult,\n    TYPED_OPERATION_ACK_MAGIC,\n",
     "    typedOperationAcknowledgements,\n    typedOperationPageAnswerV1,\n    typedOperationResult,\n    TYPED_OPERATION_ACK_MAGIC,\n"),
    (LAWS,
     """      expect(scan.acknowledgements.length).toBe(1);
    });
  });
}""",
     """      expect(scan.acknowledgements.length).toBe(1);
    });

    it("answers every declared lane's page with its own acknowledgement, and only a fault page refuses the action", () => {
      const token = fixture.shellMessageStream.token;
      for (const lane of fixture.lanes) {
        const page = typedOperationResult(shellMessage(token.receiver, encodePage(token, lane.tag, "{}")));
        const answer = typedOperationPageAnswerV1(page);
        expect(answer.acknowledgement, lane.name).toEqual(page.acknowledgement);
        expect(answer.refused, lane.name).toBe(lane.tag === fixture.faultTag);
        expect(answer.artifact, lane.name).toBe(lane.tag === 0);
      }
    });
  });
}"""),
    (WORKER,
     "type BrowserActorActionPublication = { readonly kind: \"ui-intent\" | \"app-command\"; readonly sequence: number; frames: number; readonly hostEffects: (readonly number[])[]; readonly historyPatches: (readonly number[])[] };",
     "type BrowserActorActionPublication = { readonly kind: \"ui-intent\" | \"app-command\"; readonly sequence: number; frames: number; refusals: number; readonly hostEffects: (readonly number[])[]; readonly historyPatches: (readonly number[])[] };"),
    (WORKER,
     "        const publication: BrowserActorActionPublication = { kind: request.payload.kind, sequence: request.actionSequence, frames: 0, hostEffects: [], historyPatches: [] };",
     "        const publication: BrowserActorActionPublication = { kind: request.payload.kind, sequence: request.actionSequence, frames: 0, refusals: 0, hostEffects: [], historyPatches: [] };"),
    (WORKER,
     "        if (publication.frames !== 1) throw new Error(\"action-publication-mismatch\");\n        await this.refreshDocumentSurfaces(child, () => this.assertDocumentOwnerCurrent(), command !== null || mutationCount > 0);",
     "        if (publication.refusals !== 0) throw new Error(\"action-guest-refused\");\n        if (publication.frames !== 1) throw new Error(\"action-publication-mismatch\");\n        await this.refreshDocumentSurfaces(child, () => this.assertDocumentOwnerCurrent(), command !== null || mutationCount > 0);"),
    (WORKER,
     "  private async routeTurnEffects(value: BrowserActorChildValue, mode: \"ordinary\" | \"control\" | BrowserActorActionPublication = \"ordinary\"): Promise<Readonly<{ receipts: readonly Uint8Array[]; mutations: number; publications: number; hostEffects: readonly (readonly number[])[]; historyPatches: readonly (readonly number[])[]; jobs: readonly BrowserActorSpawnedJob[]; acknowledgements: readonly BrowserActorChildValue[] }>> {",
     "  private async routeTurnEffects(value: BrowserActorChildValue, mode: \"ordinary\" | \"control\" | BrowserActorActionPublication = \"ordinary\"): Promise<Readonly<{ receipts: readonly Uint8Array[]; mutations: number; publications: number; refusals: number; hostEffects: readonly (readonly number[])[]; historyPatches: readonly (readonly number[])[]; jobs: readonly BrowserActorSpawnedJob[]; acknowledgements: readonly BrowserActorChildValue[] }>> {"),
    (WORKER,
     "    let mutations = 0,\n      publications = 0,\n      artifactPages = 0,\n",
     "    let mutations = 0,\n      publications = 0,\n      refusals = 0,\n      artifactPages = 0,\n"),
    (WORKER,
     """        if (page !== null) {
          if (page.lane === TYPED_OPERATION_LANE_FAULT) throw new Error("action-guest-refused");
          if (page.lane === TYPED_OPERATION_LANE_ARTIFACT) artifactPages += 1;
          acknowledgements.push({ tag: "message", val: { source: { tag: "shell", val: page.token.receiver }, payload: Uint8Array.from(page.acknowledgement.payload.payload) } });
          continue;
        }""",
     """        if (page !== null) {
          const answer = typedOperationPageAnswerV1(page);
          if (answer.refused) refusals += 1;
          if (answer.artifact) artifactPages += 1;
          acknowledgements.push({ tag: "message", val: { source: { tag: "shell", val: page.token.receiver }, payload: Uint8Array.from(answer.acknowledgement.payload.payload) } });
          continue;
        }"""),
    (WORKER,
     "    return { receipts, mutations, publications, hostEffects: retainedHostEffects, historyPatches, jobs, acknowledgements };\n  }\n\n  private async bindDocumentBackbone(",
     "    return { receipts, mutations, publications, refusals, hostEffects: retainedHostEffects, historyPatches, jobs, acknowledgements };\n  }\n\n  private async bindDocumentBackbone("),
    (WORKER,
     "        if (publication !== null) {\n          publication.frames += routed.publications;\n",
     "        if (publication !== null) {\n          publication.frames += routed.publications;\n          publication.refusals += routed.refusals;\n"),
    (WORKER,
     "import { driveSpawnedJob, spawnedJobCompletedEvent, TYPED_OPERATION_LANE_ARTIFACT, TYPED_OPERATION_LANE_FAULT, typedOperationResult, wireSpawnJob } from \"../../../../../🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts\";",
     "import { driveSpawnedJob, spawnedJobCompletedEvent, typedOperationPageAnswerV1, typedOperationResult, wireSpawnJob } from \"../../../../../🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts\";"),
]


def main() -> None:
    texts: dict[pathlib.Path, str] = {}
    for path, old, new in EDITS:
        text = texts.get(path) or path.read_text(encoding="utf-8")
        if new not in text:
            assert text.count(old) == 1, (path.name, old[:90])
            text = text.replace(old, new)
        texts[path] = text
    for path, text in texts.items():
        path.write_text(text, encoding="utf-8")
    print("ok")


main()
