/** 🧾 Owned root source, intake, instance tokens, surface mint and surface publication laws. */
import { container, grant, leaf, nativeInstanceFixture, type UiNodeRecord, createTypedWireSupport, type TypedWireVitest, type TypedWireDependencies } from "../🪛️typedwire-helpers/🟦️.tsx";

export async function registerOwnedIntakeAndSurfaceTests(vitest: TypedWireVitest, dependencies: TypedWireDependencies): Promise<void> {
  const { describe, expect, it, vi } = vitest;
  const { DEFAULT_UI_DOCUMENT_LIMITS } = dependencies;
  const { Buffer, close, finish, produce, retire } = await createTypedWireSupport(vitest, dependencies);

  //#region 🧾️TypedWireTests
  describe("TypedWire", () => {
    it("OwnedRootSource renders exact native roots through independent consumers and retains close until unmount", async () => {
      const { OwnedUiInstance } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts"); const { useOwnedUiView, useOwnedUiNode } = await import("../../📖️owned/🟦️.tsx");
      const { default: fixture } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️tests/🧫️fixtures/🌱️root-source/🔣️.json"); const { default: fields } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️tests/🧫️fixtures/📨️wire-operations/🔣️.json");
      const { encodePackValue } = await import("@semio-tech/framework-os"); const { render, act, cleanup } = await import("@testing-library/react"); const { createElement, StrictMode } = await import("react"); const native = await nativeInstanceFixture(); const { activation, lifetime } = native; const owner = new OwnedUiInstance(activation, lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); native.lease.bindHostRetirement(owner); const lookup = owner.beginSurfaceLookup(activation, lifetime, "window")!; expect(finish(lookup)).toBe("ready"); const source = lookup.takeResult()!; close(lookup);
      function Node({ id }: { id: number }) { const record = useOwnedUiNode(source, id); return createElement("span", null, record?.component.type === "text" ? record.component.value : "pending"); }
      function View({ label }: { label: string }) { const current = useOwnedUiView(source); return createElement("section", { "aria-label": label }, current.root === null ? "pending" : createElement(Node, { key: current.root, id: current.root })); }
      const first = render(createElement(StrictMode, null, createElement(View, { label: fixture.labels[0]! }))); const second = render(createElement(View, { label: fixture.labels[1]! }));
      const maintenance = async () => { await act(async () => { for (let n = 0; owner.maintenancePending; n++) { expect(n).toBeLessThan(10000); const current = owner.advanceMaintenance(grant); expect(current.items).toBeLessThanOrEqual(1); expect(current.bytes).toBeLessThanOrEqual(4096); } }); };
      await maintenance(); const initial = source.view; expect(source.view).toBe(initial); expect(first.getByLabelText(fixture.labels[0]!).textContent).toBe("pending");
      const publish = async (id: number, text: string, revision: number, cancelled = false) => {
        const input = produce(fields.node, draft => { draft.id = id; draft.component.value = text; }); const previous = source.view; const operations: unknown[] = previous.root === null ? [] : [{ tag: "remove", val: BigInt(previous.root) }]; operations.push({ tag: "upsert", val: { node: encodePackValue(input) } }, { tag: "set-root", val: BigInt(id) }); const authority = await native.source(operations, previous.revision, revision); const patch = owner.beginPatch(authority, source);
        for (let ordinal = 0; ordinal < authority.value.operationCount; ordinal++) { expect(patch.offer(ordinal)).toBe(true); expect(finish(patch), patch.failure ?? "native input").toBe("ready"); expect(patch.releaseInputReceipt(patch.peekInputReceipt()!)).toBe(true); expect(source.view).toBe(previous); }
        if (cancelled) { close(patch); expect(source.view).toBe(previous); return; }
        patch.finishInput(); await act(async () => { expect(finish(patch), patch.failure ?? "native publication").toBe("ready"); }); const token = patch.peekAcknowledgement()!; expect(token.value.revision).toBe(revision); const submitted = await native.answer(native.lease.submitUiAcknowledgement(authority, token, native.budget), { status: { tag: "idle" } }); expect(patch.acceptAcknowledgement(submitted.receipt)).toBe(true); close(patch); await maintenance();
      };
      const oracle = JSON.parse(JSON.stringify(produce(fixture.nodes, () => {}))); const a = fixture.nodes[0]!; const b = fixture.nodes[1]!;
      await publish(a.id, a.text, 1); expect(source.view).not.toBe(initial); expect(source.view).toBe(source.view); expect(source.view.root).toBe(a.id); expect(first.getByLabelText(fixture.labels[0]!).textContent).toBe(oracle[0].text); expect(second.getByLabelText(fixture.labels[1]!).textContent).toBe(oracle[0].text);
      const published = source.view; await publish(b.id, fixture.cancelled, 2, true); expect(source.view).toBe(published); expect(first.getByLabelText(fixture.labels[0]!).textContent).toBe(oracle[0].text);
      await publish(b.id, b.text, 2); expect(source.view.root).toBe(b.id); expect(first.getByLabelText(fixture.labels[0]!).textContent).toBe(oracle[1].text); expect(second.getByLabelText(fixture.labels[1]!).textContent).toBe(oracle[1].text);
      first.unmount(); await maintenance(); expect(second.getByLabelText(fixture.labels[1]!).textContent).toBe(oracle[1].text); native.lease.beginClose(); owner.beginClose(); let waiting = false;
      for (let n = 0; n < 100; n++) { const current = owner.closeStep(grant); expect(current.kind).not.toBe("complete"); if (current.kind === "blocked") { expect(current.phase).toBe("surface-readers"); waiting = true; break; } }
      expect(waiting).toBe(true); expect(owner.takeRetirementWitness()).toBeNull(); second.unmount(); close(owner); expect(owner.takeRetirementWitness()).not.toBeNull(); native.client.disposeAll(); cleanup();
    });

    it("OwnedIntake retains one lookup and exact native source until successful publication receipt", async () => {
      const { OwnedUiPatchIntake } = await import("../../📥️intake/🟦️.ts"); const { OwnedUiInstance } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts"); const { default: fixture } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️tests/🧫️fixtures/📥️intake/🔣️.json"); const { default: fields } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️tests/🧫️fixtures/📨️wire-operations/🔣️.json"); const { encodePackValue } = await import("@semio-tech/framework-os"); const native = await nativeInstanceFixture(); const { activation, lifetime } = native; const owner = new OwnedUiInstance(activation, lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const foreign = new OwnedUiInstance(activation, lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); native.lease.bindHostRetirement(owner); const source = await native.source([{ tag: "upsert", val: { node: encodePackValue(fields.node) } }, { tag: "set-root", val: BigInt(fields.root) }]); expect(source.value.operationCount).toBe(fixture.operationCount); expect(() => new OwnedUiPatchIntake(foreign, source)).toThrow(/owner/);
      const busy = owner.beginSurfaceLookup(activation, lifetime, "window")!; const intake = new OwnedUiPatchIntake(owner, source); expect(intake.advance({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); expect(intake.advance(grant)).toMatchObject({ kind: "blocked", phase: "intake-lookup-capacity" }); expect(intake.takeSurface()).toBeNull(); expect(source.inputRetired).toBe(false); close(busy);
      for (let n = 0; !intake.peekAcknowledgement(); n++) { expect(n).toBeLessThan(10000); const current = intake.advance(grant); expect(current.kind, intake.failure ?? "intake").not.toBe("rejected"); expect(current.items).toBeLessThanOrEqual(1); expect(current.bytes).toBeLessThanOrEqual(4096); expect(intake.takeSurface()).toBeNull(); }
      const token = intake.peekAcknowledgement()!; expect(source.inputRetired).toBe(true); expect(intake.advance(grant).kind).toBe("blocked"); native.worker.refuse = true; await expect(native.lease.submitUiAcknowledgement(source, token, native.budget)).rejects.toThrow(/refusal/); expect(intake.peekAcknowledgement()).toBe(token); expect(intake.acceptAcknowledgement(Object.create(Object.getPrototypeOf(token)))).toBe(false);
      const receipt = await native.answer(native.lease.submitUiAcknowledgement(source, token, native.budget), { status: { tag: "idle" } }); expect(intake.acceptAcknowledgement(receipt.receipt)).toBe(true); expect(intake.acceptAcknowledgement(receipt.receipt)).toBe(false); expect(finish(intake)).toBe("ready"); const view = intake.takeSurface()!; expect(view.view).toMatchObject(produce({ root: fields.root, revision: 1 }, () => {})); expect(intake.takeSurface()).toBeNull(); close(intake); expect(owner.terminalIsEmpty()).toBe(false); expect(view.view.root).toBe(fields.root); close(owner); close(foreign); native.client.disposeAll();
    });

    it("OwnedIntake drives a Nakagin-scale paged scene-lane surface patch to acknowledgement inside the credited budget", async () => {
      const { OwnedUiPatchIntake, retainedUiIntakeStepCeiling, RETAINED_UI_INTAKE_STEPS_PER_NODE } = await import("../../📥️intake/🟦️.ts"); const { OwnedUiInstance } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts");
      const { default: contract } = await import("../../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🚚️world3d-scene-lanes/🔣️.json"); const { encodePackValue } = await import("@semio-tech/framework-os");
      const { leafBytes, childrenMax, measuredFaultBytes } = contract.carrier;
      const nodes: UiNodeRecord[] = []; const laneRoots: number[] = []; const laneRecords: UiNodeRecord[][] = []; let nextId = 1; const laneBytes = Math.ceil(measuredFaultBytes / contract.lanes.length);
      for (const lane of contract.lanes) {
        const own: UiNodeRecord[] = []; let level: number[] = [];
        for (let offset = 0; offset < laneBytes; offset += leafBytes) { const id = nextId++; own.push({ ...leaf(id, `c${level.length}`), component: { type: "text", value: "x".repeat(Math.min(leafBytes, laneBytes - offset)), emphasize: null, dataAttributes: null } }); level.push(id); }
        for (let generation = 0; level.length > childrenMax; generation += 1) { const paged: number[] = []; for (let start = 0; start < level.length; start += childrenMax) { const id = nextId++; own.push(container(id, `p${generation}-${paged.length}`, level.slice(start, start + childrenMax))); paged.push(id); } level = paged; }
        const laneRoot = nextId++; own.push(container(laneRoot, lane.bodyKey, level)); laneRoots.push(laneRoot); nodes.push(...own); laneRecords.push(own);
      }
      const surfaceRoot = nextId++; nodes.push(container(surfaceRoot, "puzzle3d-main-perspective", laneRoots));
      const ops = [...nodes.map((node) => ({ tag: "upsert", val: { node: encodePackValue(node) } })), { tag: "set-root", val: BigInt(surfaceRoot) }];
      const carried = nodes.reduce((total, node) => total + (node.component.type === "text" ? node.component.value.length : 0), 0);
      expect(carried).toBeGreaterThanOrEqual(measuredFaultBytes); expect(ops.length).toBeLessThanOrEqual(DEFAULT_UI_DOCUMENT_LIMITS.maxPatchOps);
      const native = await nativeInstanceFixture(); const owner = new OwnedUiInstance(native.activation, native.lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); native.lease.bindHostRetirement(owner);
      const budget = retainedUiIntakeStepCeiling(DEFAULT_UI_DOCUMENT_LIMITS); const uiGrant = { maxItems: 256, maxBytes: 65_536 };
      const drive = async (operations: readonly unknown[], baseRevision: number, revision: number) => {
        const source = await native.source(operations, baseRevision, revision); const intake = new OwnedUiPatchIntake(owner, source);
        let phase = ""; let samePhaseRun = 0; let longestSamePhaseRun = 0; let idleRun = 0; let longestIdleRun = 0; let steps = 0; let validation = 0;
        while (!intake.peekAcknowledgement()) {
          steps += 1; expect(steps, `${phase} exhausted the credited budget`).toBeLessThanOrEqual(budget);
          const current = intake.advance(uiGrant); expect(current.kind, intake.failure ?? current.phase).not.toBe("rejected");
          if (current.phase === "validation") validation += 1;
          if (current.kind === "blocked") expect(intake.peekAcknowledgement(), current.phase).not.toBeNull();
          if (current.phase === phase) samePhaseRun += 1; else { phase = current.phase; samePhaseRun = 1; }
          longestSamePhaseRun = Math.max(longestSamePhaseRun, samePhaseRun);
          if (current.kind === "pending" && current.bytes === 0 && current.items === 0) idleRun += 1; else idleRun = 0;
          longestIdleRun = Math.max(longestIdleRun, idleRun);
        }
        const token = intake.peekAcknowledgement()!; const receipt = await native.answer(native.lease.submitUiAcknowledgement(source, token, native.budget), { status: { tag: "idle" } });
        expect(intake.acceptAcknowledgement(receipt.receipt)).toBe(true); expect(finish(intake)).toBe("ready"); const view = intake.takeSurface()!;
        intake.beginClose(); let closeSteps = 0;
        while (!intake.terminalIsEmpty()) { closeSteps += 1; expect(closeSteps).toBeLessThan(500_000); expect(intake.closeStep(uiGrant).kind, intake.failure ?? "intake-close").not.toBe("rejected"); }
        expect(closeSteps, "PluginRuntime retires an intake within PLUGIN_UI_CONTINUATION_LIMIT steps or throws plugin-ui.intake-close-budget-exhausted").toBeLessThanOrEqual(4_096);
        return { steps, longestSamePhaseRun, longestIdleRun, validation, view };
      };
      const first = await drive(ops, 0, 1);
      expect(first.longestSamePhaseRun, "a phase-name-only progress guard would reject this patch").toBeGreaterThan(4_096);
      expect(first.longestIdleRun, "the intake's own byte-aware progress detector must never fire").toBeLessThan(32);
      expect(first.steps, "a first publication mints every node and re-roots the surface, so it pays the whole-graph walk").toBeLessThanOrEqual(retainedUiIntakeStepCeiling({ maxNodes: nodes.length })); expect(first.steps).toBeGreaterThan(RETAINED_UI_INTAKE_STEPS_PER_NODE);
      expect(first.validation, "re-rooting the surface must fall to the authoritative whole-graph walk").toBeGreaterThan(10 * nodes.length);
      expect(first.view.view.root).toBe(surfaceRoot); expect(first.view.view.revision).toBe(1); const published = first.view.view.hash;
      const relane = laneRecords[0]!.map((node) => (node.component.type === "text" ? { ...node, component: { ...node.component, value: "y".repeat(node.component.value.length) } } : node));
      const second = await drive(relane.map((node) => ({ tag: "upsert", val: { node: encodePackValue(node) } })), 1, 2);
      expect(second.view.view.revision).toBe(2); expect(second.view.view.root).toBe(surfaceRoot); expect(second.view.view.hash).not.toBe(published);
      expect(second.validation, "a same-shape delta re-checks only the records it replaced").toBeLessThan(40 * relane.length);
      expect(second.steps * 10, "one lane of an unchanged 145-node document must cost a delta, not a document").toBeLessThan(first.steps);
      close(owner); native.client.disposeAll();
    });

    /** 🧩️ The law above builds UNPACKED `leafBytes` leaves, which is what a small document publishes.
     * A Nakagin-scale one does not: `section_text_chunks` (`💻️os/🔨️modules/🔌️plugin/🦀️.rs`) puts slice 0
     * in `TextProps.value` and slices 1…`packChunks - 1` in `data_attributes`, `packBytes` per leaf,
     * because a leaf per slice put the world surface over `UI_DOCUMENT_NODES` (128). Every host
     * consumer of a carrier — `PagedSurfaceView`'s lane walk and `sectionValueFromBuiltNode` alike —
     * reads the leaf back through `packedTextLeaf`, so an intake that admitted the node but dropped
     * the 32 attribute slices would hand `JSON.parse` an exact 512-byte prefix and nothing else: the
     * `Unterminated string in JSON at position 512` of ticket 26/09/09/PROCEDURAL-3D-END-TO-END. This
     * drives one packed carrier through the REAL intake and reads the published record back the way
     * `PluginRuntime.projectOwnedUiSurface` does. */
    it("OwnedIntake publishes a packed scene-lane leaf whose data-attribute slices all survive the wire", async () => {
      const { OwnedUiPatchIntake } = await import("../../📥️intake/🟦️.ts"); const { OwnedUiInstance } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts");
      const { default: contract } = await import("../../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🚚️world3d-scene-lanes/🔣️.json"); const { encodePackValue } = await import("@semio-tech/framework-os"); const { packedTextLeaf } = await import("../../../🔌️PluginRuntime/🧳️packed-text/🟦️.ts");
      const { leafBytes, packChunks, packBytes } = contract.carrier; expect(packBytes).toBe(leafBytes * packChunks);
      const lane = contract.lanes.find((entry) => entry.lane === "instances")!;
      const payload = `[${Array.from({ length: 512 }, (_, index) => `{"id":"capsule-${String(index).padStart(4, "0")}","meshId":"mesh:box","position":[${index},0,0]}`).join(",")}]`;
      expect(payload.length).toBeGreaterThan(packBytes);
      const slices: string[] = []; for (let start = 0; start < payload.length; start += leafBytes) slices.push(payload.slice(start, start + leafBytes));
      const nodes: UiNodeRecord[] = []; const laneChildren: number[] = []; let nextId = 1;
      for (let offset = 0; offset < slices.length; offset += packChunks) {
        const group = slices.slice(offset, offset + packChunks); const dataAttributes: Record<string, string> = {};
        for (let index = 1; index < group.length; index += 1) dataAttributes[String(index).padStart(2, "0")] = group[index]!;
        const id = nextId++; nodes.push({ ...leaf(id, `c${laneChildren.length}`), component: { type: "text", value: group[0]!, emphasize: null, dataAttributes: Object.keys(dataAttributes).length > 0 ? dataAttributes : null } }); laneChildren.push(id);
      }
      expect(nodes).toHaveLength(Math.ceil(slices.length / packChunks)); expect(nodes.length).toBeGreaterThan(1);
      const laneRoot = nextId++; nodes.push(container(laneRoot, lane.bodyKey, laneChildren));
      const surfaceRoot = nextId++; nodes.push(container(surfaceRoot, "puzzle3d-main-perspective", [laneRoot]));
      const operations = [...nodes.map((node) => ({ tag: "upsert", val: { node: encodePackValue(node) } })), { tag: "set-root", val: BigInt(surfaceRoot) }];

      const native = await nativeInstanceFixture(); const owner = new OwnedUiInstance(native.activation, native.lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); native.lease.bindHostRetirement(owner);
      const uiGrant = { maxItems: 256, maxBytes: 65_536 }; const source = await native.source(operations, 0, 1); const intake = new OwnedUiPatchIntake(owner, source);
      for (let step = 1; !intake.peekAcknowledgement(); step += 1) { expect(step).toBeLessThan(4_000_000); expect(intake.advance(uiGrant).kind, intake.failure ?? "intake").not.toBe("rejected"); }
      const receipt = await native.answer(native.lease.submitUiAcknowledgement(source, intake.peekAcknowledgement()!, native.budget), { status: { tag: "idle" } });
      expect(intake.acceptAcknowledgement(receipt.receipt)).toBe(true); expect(finish(intake)).toBe("ready"); const published = intake.takeSurface()!;
      intake.beginClose(); for (let step = 1; !intake.terminalIsEmpty(); step += 1) { expect(step).toBeLessThan(500_000); expect(intake.closeStep(uiGrant).kind, intake.failure ?? "intake-close").not.toBe("rejected"); }

      const readRecord = (id: number) => {
        const subscription = published.subscribeNode(id, () => {});
        try { while (owner.maintenancePending) owner.advanceMaintenance(uiGrant); const current = subscription.snapshot; expect(current?.record, `node ${id}`).toBeTruthy(); published.acknowledgeRead(subscription, current!); return current!.record!; } finally { published.unsubscribeNode(subscription); while (owner.maintenancePending) owner.advanceMaintenance(uiGrant); }
      };
      const laneRecord = readRecord(laneRoot); expect(String(laneRecord.key)).toBe(lane.bodyKey);
      let reassembled = "";
      for (const childId of laneRecord.children) { const record = readRecord(Number(childId)); expect(record.component.type).toBe("text"); reassembled += packedTextLeaf((record.component as { readonly value: string }).value, (record.component as { readonly dataAttributes?: Record<string, string> | null }).dataAttributes); }
      expect(reassembled, "a packed leaf that loses its data-attribute slices hands JSON.parse a 512-byte prefix").toBe(payload);
      expect(JSON.parse(reassembled)).toHaveLength(512);
      close(owner); native.client.disposeAll();
    });

    /** 🚚️ Wave W-S2 (ticket 26/09/02): the host's intake cost for a whole world-3d LANE SET, on the
     * shape production actually publishes. The W-P4/W-R law above builds UNPACKED `leafBytes` leaves
     * and measured 669 403 steps for a Nakagin-scale surface; the guest packs 33 slices per leaf
     * (`section_text_chunks`), so the real surface is 26 nodes (measured on the guest:
     * `Nakagin world-3d surface presented 26 nodes`) and costs 105 416 — 1.86 steps per carried byte
     * against 11.7 unpacked. That ratio is the property worth pinning: an intake priced per NODE
     * silently absorbs a shape regression (unpacking the leaves keeps the per-node cost and multiplies
     * the node count), an intake priced per carried BYTE does not, and 512-byte-per-node leaves are
     * one `if` away in the producer. Second clause: re-publishing ONE lane of an otherwise unchanged
     * document must stay a delta (W-R), measured 18 565 steps against the first publication's 105 416.
     */
    it("OwnedIntake takes a whole packed world-3d lane set in at a bounded cost per carried byte", async () => {
      const { OwnedUiPatchIntake } = await import("../../📥️intake/🟦️.ts"); const { OwnedUiInstance } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts");
      const { default: contract } = await import("../../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🚚️world3d-scene-lanes/🔣️.json"); const { encodePackValue } = await import("@semio-tech/framework-os");
      const { leafBytes, packChunks, childrenMax } = contract.carrier;
      /** 📏️ The lane byte counts the puzzle 3d editor publishes for the Nakagin Capsule Tower,
       * measured on the guest 2026-09-10 (`Nakagin world-3d surface presented 26 nodes … `). */
      const measured: Record<string, number> = { meshes: 907, instances: 55154, selection: 212, vortices: 2, attractions: 2, targetVolumes: 2, references: 2, interaction: 256, lod: 125, chunking: 40, environment: 92 };
      const carried = Object.values(measured).reduce((total, bytes) => total + bytes, 0);
      const build = (slicesPerLeaf: number) => {
        const nodes: UiNodeRecord[] = []; const laneRoots: number[] = []; const laneRecords: Record<string, UiNodeRecord[]> = {}; let nextId = 1;
        for (const [name, bytes] of Object.entries(measured)) {
          const lane = contract.lanes.find((entry) => entry.lane === name)!;
          const payload = "x".repeat(bytes); const own: UiNodeRecord[] = []; let level: number[] = [];
          const slices: string[] = []; for (let start = 0; start < payload.length; start += leafBytes) slices.push(payload.slice(start, start + leafBytes));
          for (let offset = 0; offset < slices.length; offset += slicesPerLeaf) {
            const group = slices.slice(offset, offset + slicesPerLeaf); const dataAttributes: Record<string, string> = {};
            for (let index = 1; index < group.length; index += 1) dataAttributes[String(index).padStart(2, "0")] = group[index]!;
            const id = nextId++; own.push({ ...leaf(id, `c${level.length}`), component: { type: "text", value: group[0]!, emphasize: null, dataAttributes: Object.keys(dataAttributes).length > 0 ? dataAttributes : null } }); level.push(id);
          }
          for (let generation = 0; level.length > childrenMax; generation += 1) { const paged: number[] = []; for (let start = 0; start < level.length; start += childrenMax) { const id = nextId++; own.push(container(id, `p${generation}-${paged.length}`, level.slice(start, start + childrenMax))); paged.push(id); } level = paged; }
          const laneRoot = nextId++; own.push(container(laneRoot, lane.bodyKey, level)); laneRoots.push(laneRoot); nodes.push(...own); laneRecords[name] = own;
        }
        const surfaceRoot = nextId++; nodes.push(container(surfaceRoot, "puzzle3d-main-perspective", laneRoots));
        return { nodes, surfaceRoot, laneRecords, operations: [...nodes.map((node) => ({ tag: "upsert", val: { node: encodePackValue(node) } })), { tag: "set-root", val: BigInt(surfaceRoot) }] };
      };
      const packed = build(packChunks);
      expect(packed.nodes.length, "the production surface packs into a couple of dozen nodes").toBeLessThanOrEqual(32);

      const native = await nativeInstanceFixture(); const owner = new OwnedUiInstance(native.activation, native.lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); native.lease.bindHostRetirement(owner);
      const uiGrant = { maxItems: 256, maxBytes: 65_536 };
      const drive = async (operations: readonly unknown[], baseRevision: number, revision: number) => {
        const source = await native.source(operations, baseRevision, revision); const intake = new OwnedUiPatchIntake(owner, source);
        let steps = 0;
        while (!intake.peekAcknowledgement()) { steps += 1; expect(steps).toBeLessThan(4_000_000); const current = intake.advance(uiGrant); expect(current.kind, intake.failure ?? current.phase).not.toBe("rejected"); }
        const receipt = await native.answer(native.lease.submitUiAcknowledgement(source, intake.peekAcknowledgement()!, native.budget), { status: { tag: "idle" } });
        expect(intake.acceptAcknowledgement(receipt.receipt)).toBe(true); expect(finish(intake)).toBe("ready"); const view = intake.takeSurface()!;
        intake.beginClose(); while (!intake.terminalIsEmpty()) expect(intake.closeStep(uiGrant).kind, intake.failure ?? "close").not.toBe("rejected");
        return { steps, view };
      };
      const first = await drive(packed.operations, 0, 1);
      expect(first.view.view.root).toBe(packed.surfaceRoot);
      expect(first.steps, `a packed lane set must cost a bounded number of steps per carried byte; ${first.steps} steps for ${carried} bytes`).toBeLessThanOrEqual(3 * carried);
      const relane = packed.laneRecords.instances!.map((node) => (node.component.type === "text" ? { ...node, component: { ...node.component, value: "y".repeat(node.component.value.length) } } : node));
      const second = await drive(relane.map((node) => ({ tag: "upsert", val: { node: encodePackValue(node) } })), 1, 2);
      expect(second.view.view.revision).toBe(2);
      expect(second.steps * 4, "re-publishing one lane of an unchanged document must stay a delta").toBeLessThan(first.steps);
      close(owner); native.client.disposeAll();
    });

    it("OwnedIntake cancellation retires each accepted prefix without closing the whole instance", async () => {
      const { OwnedUiPatchIntake } = await import("../../📥️intake/🟦️.ts"); const { OwnedUiInstance } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts"); const { default: fixture } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️tests/🧫️fixtures/📥️intake/🔣️.json"); const { default: fields } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️tests/🧫️fixtures/📨️wire-operations/🔣️.json"); const { encodePackValue } = await import("@semio-tech/framework-os");
      for (const prefix of fixture.cancelPrefixes) {
        const native = await nativeInstanceFixture(); const { activation, lifetime } = native; const owner = new OwnedUiInstance(activation, lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); native.lease.bindHostRetirement(owner); const source = await native.source([{ tag: "upsert", val: { node: encodePackValue(fields.node) } }, { tag: "set-root", val: BigInt(fields.root) }]); const intake = new OwnedUiPatchIntake(owner, source);
        for (let n = 0; n < prefix; n++) { const current = intake.advance(grant); expect(current.kind, intake.failure ?? "prefix").not.toBe("rejected"); if (intake.peekAcknowledgement()) break; }
        intake.beginClose(); expect(intake.closeStep({ maxItems: 1, maxBytes: 0 }).kind).toBe("blocked"); expect(intake.takeSurface()).toBeNull();
        for (let n = 0; !intake.terminalIsEmpty(); n++) { expect(n).toBeLessThan(10000); const token = intake.peekAcknowledgement(); if (token) { const receipt = await native.answer(native.lease.submitUiAcknowledgement(source, token, native.budget), { status: { tag: "idle" } }); expect(intake.acceptAcknowledgement(receipt.receipt)).toBe(true); } const current = intake.closeStep(grant); expect(current.items).toBeLessThanOrEqual(1); expect(current.bytes).toBeLessThanOrEqual(4096); }
        expect(owner.terminalIsEmpty()).toBe(false); close(owner); native.client.disposeAll();
      }
    });

    it("OwnedIntake late cancellation preserves committed notification and ACK under the shared close driver", async () => {
      const { OwnedUiPatchIntake } = await import("../../📥️intake/🟦️.ts"); const { OwnedUiInstance } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts"); const { default: fields } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️tests/🧫️fixtures/📨️wire-operations/🔣️.json"); const { encodePackValue } = await import("@semio-tech/framework-os");
      const native = await nativeInstanceFixture(); const { activation, lifetime } = native; const owner = new OwnedUiInstance(activation, lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); native.lease.bindHostRetirement(owner); const lookup = owner.beginSurfaceLookup(activation, lifetime, "window")!; expect(finish(lookup)).toBe("ready"); const surface = lookup.takeResult()!; close(lookup); let notifications = 0; const subscription = surface.subscribeView(() => { notifications++; }); while (owner.maintenancePending) owner.advanceMaintenance(grant); const before = notifications;
      const source = await native.source([{ tag: "upsert", val: { node: encodePackValue(fields.node) } }, { tag: "set-root", val: BigInt(fields.root) }]); const intake = new OwnedUiPatchIntake(owner, source);
      for (let n = 0; surface.view.revision === 0; n++) { expect(n).toBeLessThan(10000); const current = intake.advance(grant); expect(current.kind, intake.failure ?? "publication").not.toBe("rejected"); }
      expect(intake.peekAcknowledgement()).toBeNull(); expect(notifications).toBe(before); native.lease.beginClose(); owner.beginClose(); intake.beginClose();
      for (let n = 0; !intake.peekAcknowledgement(); n++) { expect(n).toBeLessThan(10000); expect(owner.closeStep(grant).kind).not.toBe("complete"); const current = intake.closeStep(grant); expect(current.bytes).toBeLessThanOrEqual(4096); }
      const token = intake.peekAcknowledgement()!; expect(notifications).toBeGreaterThan(before); expect(owner.takeRetirementWitness()).toBeNull(); native.worker.refuse = true; await expect(native.lease.submitUiAcknowledgement(source, token, native.budget)).rejects.toThrow(/refusal/); expect(intake.peekAcknowledgement()).toBe(token); expect(intake.closeStep(grant).kind).toBe("blocked");
      const receipt = await native.answer(native.lease.submitUiAcknowledgement(source, token, native.budget), { status: { tag: "idle" } }); expect(intake.acceptAcknowledgement(receipt.receipt)).toBe(true); surface.unsubscribeNode(subscription);
      for (let n = 0; !intake.terminalIsEmpty() || !owner.terminalIsEmpty(); n++) { expect(n).toBeLessThan(10000); if (!owner.terminalIsEmpty()) owner.closeStep(grant); if (!intake.terminalIsEmpty()) intake.closeStep(grant); }
      expect(intake.takeSurface()).toBeNull(); expect(owner.takeRetirementWitness()).not.toBeNull(); native.client.disposeAll();
    });

    it("OwnedIntake preserves an injected committed close rejection and its exact retained owner", async () => {
      const { OwnedUiPatchIntake } = await import("../../📥️intake/🟦️.ts"); const { OwnedUiInstance, OwnedUiInstancePatch } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts"); const { default: fixture } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️tests/🧫️fixtures/🚨️intake-close-fault/🔣️.json"); const { default: fields } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️tests/🧫️fixtures/📨️wire-operations/🔣️.json"); const { encodePackValue } = await import("@semio-tech/framework-os"); const native = await nativeInstanceFixture(); const { activation, lifetime } = native; const owner = new OwnedUiInstance(activation, lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); native.lease.bindHostRetirement(owner); const lookup = owner.beginSurfaceLookup(activation, lifetime, "window")!; expect(finish(lookup)).toBe("ready"); const surface = lookup.takeResult()!; close(lookup); const source = await native.source([{ tag: "upsert", val: { node: encodePackValue(fields.node) } }, { tag: "set-root", val: BigInt(fields.root) }]); const intake = new OwnedUiPatchIntake(owner, source);
      for (let n = 0; surface.view.revision === 0; n++) { expect(n).toBeLessThan(10000); expect(intake.advance(grant).kind).not.toBe("rejected"); } intake.beginClose();
      const fault = vi.spyOn(OwnedUiInstancePatch.prototype, "closeStep").mockReturnValueOnce({ kind: "rejected", phase: fixture.phase, items: fixture.items, bytes: fixture.bytes });
      try { const actual = intake.closeStep(grant); expect(actual).toEqual(produce({ kind: fixture.kind, phase: fixture.phase, items: fixture.items, bytes: fixture.bytes }, () => {})); expect(intake.failure).toContain(fixture.phase); expect(intake.terminalIsEmpty()).toBe(!fixture.retainsOwner); expect(intake.peekAcknowledgement() !== null).toBe(fixture.issuesAcknowledgement); expect(intake.takeSurface()).toBeNull(); } finally { fault.mockRestore(); }
      for (let n = 0; !intake.terminalIsEmpty(); n++) { expect(n).toBeLessThan(10000); const token = intake.peekAcknowledgement(); if (token) { const receipt = await native.answer(native.lease.submitUiAcknowledgement(source, token, native.budget), { status: { tag: "idle" } }); expect(intake.acceptAcknowledgement(receipt.receipt)).toBe(true); } intake.closeStep(grant); }
      expect(intake.failure).toContain(fixture.phase); close(owner); native.client.disposeAll();
    });

    it("OwnedIntake preserves failed notification ownership and permits exact consumer retry before later publication", async () => {
      const { OwnedUiPatchIntake } = await import("../../📥️intake/🟦️.ts"); const { OwnedUiInstance } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts"); const { default: fixture } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️tests/🧫️fixtures/🔔️intake-notification/🔣️.json"); const { default: fields } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️tests/🧫️fixtures/📨️wire-operations/🔣️.json"); const { encodePackValue } = await import("@semio-tech/framework-os"); const native = await nativeInstanceFixture(); const { activation, lifetime } = native; const owner = new OwnedUiInstance(activation, lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); native.lease.bindHostRetirement(owner); const lookup = owner.beginSurfaceLookup(activation, lifetime, "window")!; expect(finish(lookup)).toBe("ready"); const surface = lookup.takeResult()!; close(lookup); const events: string[] = []; let fail = true;
      const subscription: import("../../📖️owned/🟦️.tsx").OwnedUiReadSubscription = surface.subscribeNode(fields.root, () => { const snapshot = subscription.snapshot!; if (snapshot.version === 1 && fail) { fail = false; events.push("rejected"); throw new Error("Temporary consumer failure"); } events.push(String(snapshot.version)); surface.acknowledgeRead(subscription, snapshot); }); while (owner.maintenancePending) owner.advanceMaintenance(grant);
      const publish = async (operations: readonly unknown[], revision: number) => { const source = await native.source(operations, revision - 1, revision); const intake = new OwnedUiPatchIntake(owner, source); for (let n = 0; !intake.peekAcknowledgement(); n++) { expect(n).toBeLessThan(10000); expect(intake.advance(grant).kind, intake.failure ?? "notification").not.toBe("rejected"); } const token = intake.peekAcknowledgement()!; const receipt = await native.answer(native.lease.submitUiAcknowledgement(source, token, native.budget), { status: { tag: "idle" } }); expect(intake.acceptAcknowledgement(receipt.receipt)).toBe(true); expect(finish(intake)).toBe("ready"); expect(intake.takeSurface()).toBe(surface); close(intake); };
      await publish([{ tag: "upsert", val: { node: encodePackValue(fields.node) } }, { tag: "set-root", val: BigInt(fields.root) }], 1); expect(events).toEqual(fixture.events.slice(0, 2)); expect(owner.terminalIsEmpty()).toBe(fixture.retireBeforeRetry); expect(surface.retryNotification(subscription)).toBe(fixture.retryAccepted); expect(surface.retryNotification(subscription)).toBe(fixture.duplicateRetryAccepted); while (owner.maintenancePending) owner.advanceMaintenance(grant); expect(events).toEqual(fixture.events.slice(0, 3));
      await publish([{ tag: "set-component", val: { node: BigInt(fields.root), component: encodePackValue({ type: "text", value: "Recovered / Wiederhergestellt" }) } }], 2); expect(events).toEqual(produce(fixture.events, () => {})); surface.unsubscribeNode(subscription); close(owner); native.client.disposeAll();
    });

    it("OwnedInstance old patch facades cannot read or release a successor input receipt", async () => {
      const { OwnedUiInstance } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts"); const { default: fields } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️tests/🧫️fixtures/📨️wire-operations/🔣️.json"); const { encodePackValue } = await import("@semio-tech/framework-os");
      const native = await nativeInstanceFixture(); const { activation, lifetime } = native; const instance = new OwnedUiInstance(activation, lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); native.lease.bindHostRetirement(instance);
      const lookup = instance.beginSurfaceLookup(activation, lifetime, "window")!; expect(finish(lookup)).toBe("ready"); const view = lookup.takeResult()!; close(lookup);
      const old = instance.beginPatch(await native.source([]), view); close(old); const next = instance.beginPatch(await native.source([{ tag: "upsert", val: { node: encodePackValue(fields.node) } }]), view); expect(next.offer(0)).toBe(true); expect(finish(next)).toBe("ready"); const receipt = next.peekInputReceipt()!;
      expect(() => old.peekInputReceipt()).toThrow(/retired/); expect(() => old.releaseInputReceipt(receipt)).toThrow(/retired/); expect(next.peekInputReceipt()).toBe(receipt); expect(next.releaseInputReceipt(receipt)).toBe(true); close(next); close(instance); native.client.disposeAll();
    });

    it("OwnedInstance acknowledgement token rejects forged roots without consulting public properties", async () => {
      const { OwnedUiPatchAcknowledgement } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts"); let reads = 0; const source = Object.freeze({}); const forged = { get value() { reads++; throw new Error("Forged value access"); }, matches() { reads++; return true; } };
      expect(OwnedUiPatchAcknowledgement.matches(forged, source)).toBe(false); expect(() => Reflect.construct(OwnedUiPatchAcknowledgement, [forged, source, forged])).toThrow(/authority/); expect(reads).toBe(0);
    });

    it("OwnedInstance input-retirement token rejects forged ordinals and preserves cancelled exact source ownership", async () => {
      const { OwnedUiInstance, OwnedUiPatchInputRetirement } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts"); const { encodePackValue } = await import("@semio-tech/framework-os"); const { default: fields } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️tests/🧫️fixtures/📨️wire-operations/🔣️.json");
      let reads = 0; const forged = { get ordinal() { reads++; return 0; }, matches() { reads++; return true; } }; expect(OwnedUiPatchInputRetirement.matches(forged, {}, 0, {})).toBe(false); expect(() => Reflect.construct(OwnedUiPatchInputRetirement, [forged, {}, 0, {}])).toThrow(/authority/); expect(reads).toBe(0);
      const native = await nativeInstanceFixture(); const { activation, lifetime } = native; const owner = new OwnedUiInstance(activation, lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); native.lease.bindHostRetirement(owner); const lookup = owner.beginSurfaceLookup(activation, lifetime, "window")!; expect(finish(lookup)).toBe("ready"); const surface = lookup.takeResult()!; close(lookup);
      const bytes = encodePackValue(fields.node); const operation = { tag: "upsert", val: { node: bytes } }; const source = await native.source([operation]); const patch = owner.beginPatch(source, surface); expect(patch.offer(0)).toBe(true); expect(patch.peekInputReceipt()).toBeNull(); native.lease.beginClose(); patch.beginClose(); expect(patch.closeStep({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); expect(patch.peekInputReceipt()).toBeNull();
      let waiting = false; for (let i = 0; i < 500_000; i++) { const current = patch.closeStep(grant); expect(current.bytes).toBeLessThanOrEqual(4096); if (current.kind === "blocked") { expect(current.phase).toBe("instance-input-retirement"); waiting = true; break; } if (current.kind === "complete") throw new Error("Cancelled input was discarded before exact native release"); } expect(waiting).toBe(true);
      const token = patch.peekInputReceipt()!; expect(token.ordinal).toBe(0); expect(patch.peekInputReceipt()).toBe(token); expect(OwnedUiPatchInputRetirement.matches(token, source, 0, operation)).toBe(true); expect(OwnedUiPatchInputRetirement.matches(token, source, 1, operation)).toBe(false); expect(OwnedUiPatchInputRetirement.matches(token, source, 0, { ...operation })).toBe(false); expect(OwnedUiPatchInputRetirement.matches(token, {}, 0, operation)).toBe(false); expect(patch.releaseInputReceipt(Object.create(OwnedUiPatchInputRetirement.prototype))).toBe(false); expect(patch.peekInputReceipt()).toBe(token); expect(patch.releaseInputReceipt(token)).toBe(true); expect(patch.releaseInputReceipt(token)).toBe(false); expect(patch.peekInputReceipt()).toBeNull(); close(patch); close(owner); native.client.disposeAll();
    });

    it("OwnedInstance input acceptance is privately minted only after successful wire admission", async () => {
      const { OwnedUiInstance, OwnedUiPatchInputAcceptance } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts"); let reads = 0; const forged = { get ordinal() { reads++; return 0; }, matches() { reads++; return true; } }; expect(OwnedUiPatchInputAcceptance.matches(forged, {}, 0, {})).toBe(false); expect(() => Reflect.construct(OwnedUiPatchInputAcceptance, [forged, {}, 0, {}])).toThrow(/authority/); expect(reads).toBe(0);
      const native = await nativeInstanceFixture(); const { activation, lifetime } = native; const owner = new OwnedUiInstance(activation, lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); native.lease.bindHostRetirement(owner); const lookup = owner.beginSurfaceLookup(activation, lifetime, "window")!; expect(finish(lookup)).toBe("ready"); const surface = lookup.takeResult()!; close(lookup);
      const allocation = new Uint8Array(10); const partial = allocation.subarray(2, 4); const operation = { tag: "upsert", val: { node: partial } }; const source = await native.source([operation]); const patch = owner.beginPatch(source, surface); expect(() => patch.offer(0)).toThrow("Native ownership requires an entire non-shared admitted buffer"); expect(allocation.byteLength).toBe(10); expect(patch.peekInputReceipt()).toBeNull(); expect(source.inputRetired).toBe(false); close(patch); expect(patch.peekInputReceipt()).toBeNull(); close(owner); native.client.disposeAll();
    });

    it("OwnedInstance retirement witness requires exact terminal descendants and transfers once after revocation", async () => {
      const { OwnedUiInstance, OwnedUiInstanceRetirement } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts"); const { default: fixture } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️tests/🧫️fixtures/🪪️instance-owner/🔣️.json");
      let active = true; const activation = Object.freeze({ actorId: fixture.actor, activationGeneration: 9n, assertActive() { if (!active) throw new Error("Revoked activation"); }, async turn() { return {}; } }); const lifetime = Object.freeze({ activationGeneration: 9n, instanceId: fixture.instanceId, guestLifetime: BigInt(fixture.guestLifetimes[0]!) }); const nextLife = produce(lifetime, draft => { draft.guestLifetime = BigInt(fixture.guestLifetimes[1]!); });
      const owner = new OwnedUiInstance(activation, lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const foreign = new OwnedUiInstance(activation, lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 });
      const lookup = owner.beginSurfaceLookup(activation, lifetime, "window")!; expect(finish(lookup)).toBe("ready"); const surface = lookup.takeResult()!; close(lookup); const reader = surface.subscribeNode(7, () => {}); while (owner.maintenancePending) owner.advanceMaintenance(grant);
      expect(owner.takeRetirementWitness()).toBeNull(); active = false; owner.beginClose(); expect(owner.closeStep({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); expect(owner.takeRetirementWitness()).toBeNull();
      let blocked = false; for (let i = 0; i < 100; i++) { const current = owner.closeStep(grant); if (current.kind === "blocked") { blocked = true; break; } } expect(blocked).toBe(true); expect(owner.takeRetirementWitness()).toBeNull(); surface.unsubscribeNode(reader); close(owner);
      const witness = owner.takeRetirementWitness(); expect(witness).not.toBeNull(); expect(owner.takeRetirementWitness()).toBeNull(); expect(OwnedUiInstanceRetirement.matches(witness, owner, activation, lifetime)).toBe(true); expect(OwnedUiInstanceRetirement.matches(witness, foreign, activation, lifetime)).toBe(false); expect(OwnedUiInstanceRetirement.matches(witness, owner, Object.freeze({ ...activation }), lifetime)).toBe(false); expect(OwnedUiInstanceRetirement.matches(witness, owner, activation, nextLife)).toBe(false);
      let reads = 0; const forged = { isRetired() { reads++; return true; }, get owner() { reads++; throw new Error("Forged retirement access"); } }; expect(OwnedUiInstanceRetirement.matches(forged, owner, activation, lifetime)).toBe(false); expect(() => Reflect.construct(OwnedUiInstanceRetirement, [forged, owner, activation, lifetime])).toThrow(/authority/); expect(reads).toBe(0); close(foreign); expect(OwnedUiInstanceRetirement.matches(foreign.takeRetirementWitness(), owner, activation, lifetime)).toBe(false);
    });

    it("OwnedInstance cancels one-cell lookups and rejects forged facades at the native name boundary", async () => {
      const { OwnedUiInstance, OwnedUiInstanceSurface, OwnedUiSurfaceLookup } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts"); const { default: fixture } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️tests/🧫️fixtures/🪪️instance-owner/🔣️.json");
      const activation = Object.freeze({ actorId: fixture.actor, activationGeneration: 9n, assertActive() {}, async turn() { return {}; } }); const lifetime = { activationGeneration: 9n, instanceId: 7, guestLifetime: 13n }; const instance = new OwnedUiInstance(activation, lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const independent = new OwnedUiInstance(activation, lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 });
      const surfaces: import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts").OwnedUiInstanceSurface[] = [];
      for (const name of [...fixture.surfaceNames, "😀".repeat(fixture.nativeSurfaceMaximumBytes / 4)]) { const lookup = instance.beginSurfaceLookup(activation, lifetime, name)!; expect(finish(lookup)).toBe("ready"); surfaces.push(lookup.takeResult()!); close(lookup); }
      expect(() => instance.beginSurfaceLookup(activation, lifetime, "😀".repeat(fixture.nativeSurfaceMaximumBytes / 4 + 1))).toThrow(/capacity/);
      let reads = 0; const forged = { get owner() { reads++; throw new Error("Forged owner access"); } }; expect(() => Reflect.construct(OwnedUiInstanceSurface, [forged])).toThrow(/authority/); expect(() => Reflect.construct(OwnedUiSurfaceLookup, [forged])).toThrow(/authority/); expect(reads).toBe(0);
      const pending = instance.beginSurfaceLookup(activation, lifetime, "unpublished")!; expect(pending.advance(grant)).toMatchObject({ kind: "pending", bytes: 2112 }); instance.beginClose(); expect(instance.closeStep({ maxItems: 1, maxBytes: 0 }).kind).toBe("blocked"); close(instance); expect(pending.terminalIsEmpty()).toBe(true); expect(pending.takeResult()).toBeNull(); expect(() => surfaces[0]!.view).toThrow(/retired/); close(independent);
    });

    it("OwnedInstance binds native patch source to issued ACK and retries the exact pair after revocation", async () => {
      const { OwnedUiInstance, OwnedUiPatchAcknowledgement, OwnedUiInstanceRetirement } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts"); const { default: fields } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️tests/🧫️fixtures/📨️wire-operations/🔣️.json"); const { encodePackValue } = await import("@semio-tech/framework-os");
      const native = await nativeInstanceFixture(); const { activation, lifetime } = native; const owner = new OwnedUiInstance(activation, lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const foreign = new OwnedUiInstance(activation, lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); native.lease.bindHostRetirement(owner);
      const lookup = owner.beginSurfaceLookup(activation, lifetime, "window")!; expect(finish(lookup)).toBe("ready"); const surface = lookup.takeResult()!; close(lookup); const bytes = encodePackValue(fields.node); const source = await native.source([{ tag: "upsert", val: { node: bytes } }, { tag: "set-root", val: BigInt(fields.root) }]);
      expect(() => foreign.beginPatch(source, surface)).toThrow(/owner/); const patch = owner.beginPatch(source, surface); expect(patch.peekAcknowledgement()).toBeNull(); let notifications = 0; const reader = surface.subscribeNode(fields.root, () => { notifications++; }); while (owner.maintenancePending) owner.advanceMaintenance(grant); const initial = reader.snapshot;
      for (let ordinal = 0; ordinal < source.value.operationCount; ordinal++) { expect(patch.offer(ordinal)).toBe(true); expect(finish(patch)).toBe("ready"); expect(patch.releaseInputReceipt(patch.peekInputReceipt()!)).toBe(true); expect(patch.peekAcknowledgement()).toBeNull(); }
      patch.finishInput(); expect(finish(patch)).toBe("ready"); expect(surface.view.revision).toBe(1); expect(reader.snapshot).not.toBe(initial); expect(notifications).toBeGreaterThan(1); const token = patch.peekAcknowledgement(); if (!token) throw new Error("Missing owned publication token"); expect(patch.peekAcknowledgement()).toBe(token); expect(OwnedUiPatchAcknowledgement.matches(token, source)).toBe(true); expect(OwnedUiPatchAcknowledgement.matches(token, {})).toBe(false); expect(token.value).toMatchObject({ actor: activation.actorId, instance: 7, surface: "window", revision: 1, lifetime }); expect(bytes.byteLength).toBe(0);
      expect(token.value.receipt).toEqual(source.value.receipt); expect(token.value.receipt.patchSequence).toBe(51n); expect(token.value.receipt.patchSequence).not.toBe(BigInt(token.value.revision)); expect(Object.isFrozen(token.value.receipt)).toBe(true); expect(Object.isFrozen(token.value.receipt.lifetime)).toBe(true);
      const closeRequest = native.lease.beginClose(); owner.beginClose(); expect(() => activation.assertActive()).toThrow(/revoked/); expect(owner.closeStep(grant).kind).toBe("blocked"); expect(owner.takeRetirementWitness()).toBeNull(); expect(patch.acceptAcknowledgement({ success: true })).toBe(false); native.worker.refuse = true; await expect(native.lease.submitUiAcknowledgement(source, token, native.budget)).rejects.toThrow(/refusal/); expect(patch.peekAcknowledgement()).toBe(token);
      await expect(native.answer(native.lease.submitUiAcknowledgement(source, token, native.budget), { status: { tag: "fault", val: "fixture" } })).rejects.toThrow(/not-admitted/); expect(patch.peekAcknowledgement()).toBe(token); const submitted = await native.answer(native.lease.submitUiAcknowledgement(source, token, native.budget), { status: { tag: "idle" } }); expect(patch.acceptAcknowledgement(submitted.receipt)).toBe(true); expect(patch.acceptAcknowledgement(submitted.receipt)).toBe(false); expect(patch.peekAcknowledgement()).toBeNull();
      const { encodeActorInstanceLifecycle } = await import("../../../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🚪️lifetime/🟦️.ts"); const accepted = { kind: "accepted" as const, lifetime, requestSequence: closeRequest.requestSequence, closeGeneration: 17n }; const retired = { ...accepted, kind: "retired" as const };
      await native.answer(native.lease.close(native.budget), { status: { tag: "idle" }, lifecycleReceipt: encodeActorInstanceLifecycle(accepted) }); await native.answer(native.lease.acknowledge(accepted, native.budget), { status: { tag: "idle" }, lifecycleReceipt: encodeActorInstanceLifecycle(retired) }); close(foreign); const foreignWitness = foreign.takeRetirementWitness()!; await expect(native.lease.acknowledge(retired, native.budget, foreignWitness)).rejects.toThrow(/host-retirement-pending/); expect(owner.takeRetirementWitness()).toBeNull();
      surface.unsubscribeNode(reader); close(owner); const witness = owner.takeRetirementWitness()!; expect(OwnedUiInstanceRetirement.matches(witness, owner, activation, lifetime)).toBe(true); native.worker.refuse = true; await expect(native.lease.acknowledge(retired, native.budget, witness)).rejects.toThrow(/refusal/); expect(native.lease.pendingReceipt).toEqual(retired); await native.answer(native.lease.acknowledge(retired, native.budget, witness), { status: { tag: "idle" } }); expect(native.lease.progress().kind).toBe("complete"); native.client.dispose("native-ui"); native.client.disposeAll();
    });

    it("OwnedInstance refuses the same native source on an independently owned same-lifetime surface", async () => {
      const { OwnedUiInstance } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts"); const native = await nativeInstanceFixture(); const { activation, lifetime } = native;
      const owner = new OwnedUiInstance(activation, lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const foreign = new OwnedUiInstance(activation, lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); native.lease.bindHostRetirement(owner); const source = await native.source([]); const lookup = foreign.beginSurfaceLookup(activation, lifetime, "window")!; expect(finish(lookup)).toBe("ready"); const facade = lookup.takeResult()!; close(lookup);
      expect(() => foreign.beginPatch(source, facade)).toThrow(/owner/); expect(facade.view.revision).toBe(0); close(foreign); close(owner); native.client.disposeAll();
    });

    it("OwnedSurfaceMint refuses forged subscription detachment and private patch source access", async () => {
      const { OwnedUiSurface, OwnedUiSurfacePatch } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🖼️surface/🟦️.ts");
      const surface = new OwnedUiSurface({ actor: "mint", instance: 1, surface: "window" }, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const real = surface.subscribeNode(7, () => {}); while (surface.maintenancePending) surface.advanceMaintenance(grant); const token = real.snapshot;
      const prototype: unknown = Object.getPrototypeOf(real); if (!prototype || typeof prototype !== "object" || !("constructor" in prototype) || typeof prototype.constructor !== "function") throw new Error("Missing reflected constructor");
      let rejected = false;
      try { const forged: typeof real = Reflect.construct(prototype.constructor, [{ owner: surface, active: true, previous: null, next: null, failureQueued: false, queued: false, queueNext: null, lease: null, sceneClose: [null,null,null,null], sceneActive: [null,null,null,null] }]); surface.unsubscribeNode(forged); } catch { rejected = true; }
      surface.beginClose(); expect(surface.closeStep(grant)).toMatchObject({ kind: "blocked", phase: "surface-readers" }); expect(rejected).toBe(true); expect(real.snapshot).toBe(token);
      let reads = 0; const source = { get nodes() { reads++; throw new Error("Forged source access"); } };
      expect(() => Reflect.construct(OwnedUiSurfacePatch, [surface, source, 1, DEFAULT_UI_DOCUMENT_LIMITS])).toThrow(/authority/); expect(reads).toBe(0);
      surface.unsubscribeNode(real); close(surface);
    });

    it("OwnedSurface publishes a validated hashed flat root before ACK and retires exact subscription owners", async () => {
      const { OwnedUiSurface } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🖼️surface/🟦️.ts");
      const { OwnedUiOperation } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/🟦️.ts");
      const { RetainedUiTypedCursor } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️.ts");
      const { default: fixture } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️tests/🧫️fixtures/🗺️owned-surface/🔣️.json");
      const { default: fields } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️tests/🧫️fixtures/🏷️fields/🔣️.json");
      const { encodePackValue } = await import("@semio-tech/framework-os");
      const surface = new OwnedUiSurface(fixture.identity, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const independent = new OwnedUiSurface(fixture.identity, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 });
      let notices = 0; const subscription = surface.subscribeNode(fixture.root, () => notices++); const foreign = independent.subscribeNode(fixture.root, () => {});
      expect(subscription.snapshot).toBeNull(); while (surface.maintenancePending) surface.advanceMaintenance(grant); expect(subscription.snapshot!.record).toBeUndefined(); expect(notices).toBe(1);
      const input = new RetainedUiTypedCursor(encodePackValue({ ...fields.node, component: { type: "text", value: fixture.label } }), "node"); expect(finish(input)).toBe("ready"); const payload = input.takeResult()!; const record = payload.value;
      const patch = surface.beginPatch(0, 1); patch.pushOperation(OwnedUiOperation.upsert(payload)); retire(payload.beginClose()); close(input); expect(finish(patch)).toBe("ready"); expect(patch.takeAcknowledgement()).toBeNull();
      patch.pushOperation(OwnedUiOperation.setRoot(fixture.root)); expect(finish(patch)).toBe("ready"); patch.finishInput();
      for (;;) { const before = notices; const result = patch.advance(grant); expect(notices - before).toBeLessThanOrEqual(1); expect(result.bytes).toBeLessThanOrEqual(4096); expect(result.items).toBeLessThanOrEqual(1); if (surface.view.revision === 0) expect(subscription.snapshot!.record).toBeUndefined(); if (result.kind === "ready") break; if (result.kind === "rejected") throw new Error(patch.failure ?? "Surface rejected"); }
      expect(surface.view.revision).toBe(1); expect(surface.view.root).toBe(fixture.root); expect(subscription.snapshot!.record).toEqual(record); expect(independent.view.revision).toBe(0);
      const bytes = Buffer.from(JSON.stringify(produce({ surface: fixture.identity.surface, revision: 1, root: fixture.root, nodes: [record], layoutEpoch: "0" }, () => {}))); let hash = 0x811c9dc5; for (const byte of bytes) hash = Math.imul(hash ^ byte, 0x01000193) >>> 0;
      expect(surface.view.hash).toBe(`${hash.toString(16)}:1`); expect(patch.takeAcknowledgement()).toEqual({ ...fixture.identity, revision: 1, hash: surface.view.hash }); expect(patch.takeAcknowledgement()).toBeNull(); close(patch);
      surface.acknowledgeRead(subscription, subscription.snapshot!); surface.acknowledgeRead(foreign, subscription.snapshot!); while (surface.maintenancePending) surface.advanceMaintenance(grant);
      expect(() => surface.beginPatch(0, 2)).toThrow(); const unchanged = surface.view; const invalid = surface.beginPatch(1, 2); invalid.pushOperation(OwnedUiOperation.setRoot(8)); expect(finish(invalid)).toBe("ready"); invalid.finishInput(); expect(finish(invalid)).toBe("rejected"); expect(surface.view).toBe(unchanged); expect(invalid.takeAcknowledgement()).toBeNull(); close(invalid);
      surface.beginClose(); expect(surface.closeStep(grant)).toMatchObject({ kind: "blocked", phase: "surface-readers" }); expect(subscription.snapshot!.record).toEqual(record);
      surface.unsubscribeNode(subscription); expect(surface.maintenancePending).toBe(true); expect(surface.closeStep({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); close(surface); expect(() => subscription.snapshot).toThrow(); independent.unsubscribeNode(foreign); close(independent);
    });

    it("OwnedSurface React reads initialize incrementally and exact unmount releases the final published byte owner", async () => {
      const { OwnedUiSurface } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🖼️surface/🟦️.ts");
      const { OwnedUiOperation } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/🟦️.ts");
      const { RetainedUiTypedCursor } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️.ts");
      const { useOwnedUiNode } = await import("../../📖️owned/🟦️.tsx");
      const { default: fields } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️tests/🧫️fixtures/🏷️fields/🔣️.json");
      const { encodePackValue } = await import("@semio-tech/framework-os"); const { render, act, cleanup } = await import("@testing-library/react"); const { createElement } = await import("react");
      const surface = new OwnedUiSurface({ actor: "DOM", instance: 7, surface: "window" }, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 });
      const decoder = new RetainedUiTypedCursor(encodePackValue(fields.node), "node"); expect(finish(decoder)).toBe("ready"); const payload = decoder.takeResult()!;
      const seed = surface.beginPatch(0, 1); seed.pushOperation(OwnedUiOperation.upsert(payload)); retire(payload.beginClose()); close(decoder); expect(finish(seed)).toBe("ready"); seed.pushOperation(OwnedUiOperation.setRoot(fields.node.id)); expect(finish(seed)).toBe("ready"); seed.finishInput(); expect(finish(seed)).toBe("ready"); expect(seed.takeAcknowledgement()?.revision).toBe(1); close(seed);
      let last: import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️.ts").RetainedUiNodeRecord | undefined;
      function View() { const record = useOwnedUiNode(surface, fields.node.id); last = record; return createElement("span", { "aria-label": "Ansicht / View" }, record?.component.type === "surface" ? String(record.component.doc.bytes.byteAt(0)) : record?.component.type === "text" ? record.component.value : "pending"); }
      const root = render(createElement(View)); expect(root.getByLabelText("Ansicht / View").textContent).toBe("pending");
      await act(async () => { while (surface.maintenancePending) surface.advanceMaintenance(grant); }); expect(root.getByLabelText("Ansicht / View").textContent).toBe("3");
      const component = last!.component; if (component.type !== "surface") throw new Error("Expected captured bytes"); const oldBytes = component.doc.bytes;
      const input = new RetainedUiTypedCursor(encodePackValue({ type: "text", value: "Neu / New" }), "component"); expect(finish(input)).toBe("ready"); const field = input.takeResult()!;
      const patch = surface.beginPatch(1, 2); patch.pushOperation(OwnedUiOperation.field(fields.node.id, { field: "component", payload: field })); retire(field.beginClose()); close(input); expect(finish(patch)).toBe("ready"); patch.finishInput();
      await act(async () => { expect(finish(patch)).toBe("ready"); }); expect(root.getByLabelText("Ansicht / View").textContent).toBe("Neu / New"); expect(oldBytes.byteAt(0)).toBe(3); expect(patch.takeAcknowledgement()?.revision).toBe(2); close(patch);
      await act(async () => { while (surface.maintenancePending) surface.advanceMaintenance(grant); }); expect(() => oldBytes.byteAt(0)).toThrow();
      surface.beginClose(); expect(surface.closeStep(grant)).toMatchObject({ kind: "blocked", phase: "surface-readers" }); root.unmount(); close(surface); cleanup();
    });

    it("OwnedSurface notifications use an exact cutoff across unsubscribe, reentrant subscribe and thrown callbacks", async () => {
      const { OwnedUiSurface } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🖼️surface/🟦️.ts");
      const { OwnedUiOperation } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/🟦️.ts");
      const { RetainedUiTypedCursor } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️.ts");
      const { default: fields } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️tests/🧫️fixtures/🏷️fields/🔣️.json");
      const { encodePackValue } = await import("@semio-tech/framework-os");
      const surface = new OwnedUiSurface({ actor: "notices", instance: 1, surface: "window" }, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const events: string[] = [];
      let late: import("../../📖️owned/🟦️.tsx").OwnedUiReadSubscription | null = null;
      let failedOnce = false;
      const first = surface.subscribeNode(fields.node.id, () => { events.push(`a:${surface.view.revision}`); if (surface.view.revision === 1 && !failedOnce) { failedOnce = true; surface.unsubscribeNode(second); late = surface.subscribeNode(fields.node.id, () => events.push(`c:${surface.view.revision}`)); throw new Error("intentional observer failure"); } if (first.snapshot) surface.acknowledgeRead(first, first.snapshot); });
      const second = surface.subscribeNode(fields.node.id, () => events.push(`b:${surface.view.revision}`)); while (surface.maintenancePending) surface.advanceMaintenance(grant);
      const decoder = new RetainedUiTypedCursor(encodePackValue(fields.node), "node"); expect(finish(decoder)).toBe("ready"); const payload = decoder.takeResult()!; const patch = surface.beginPatch(0, 1); patch.pushOperation(OwnedUiOperation.upsert(payload)); retire(payload.beginClose()); close(decoder); expect(finish(patch)).toBe("ready"); patch.pushOperation(OwnedUiOperation.setRoot(fields.node.id)); expect(finish(patch)).toBe("ready"); patch.finishInput();
      for (;;) { const before = events.length; const step = patch.advance(grant); expect(events.length - before).toBeLessThanOrEqual(1); if (step.kind === "ready") break; if (step.kind === "rejected") throw new Error(patch.failure!); }
      expect(events).toEqual(["a:0", "b:0", "a:1"]); expect(surface.notificationFailures).toBe(1); expect(patch.takeAcknowledgement()?.revision).toBe(1); close(patch);
      while (surface.maintenancePending) surface.advanceMaintenance(grant); expect(events).toEqual(["a:0", "b:0", "a:1", "c:1"]);
      expect(surface.takeNotificationFailure()).toEqual({ subscription: first, reason: "callback-threw" }); expect(surface.takeNotificationFailure()).toBeNull(); expect(surface.retryNotification(first)).toBe(true);
      while (surface.maintenancePending) surface.advanceMaintenance(grant); expect(events).toEqual(["a:0", "b:0", "a:1", "c:1", "a:1"]); expect(surface.retryNotification(first)).toBe(false); expect(first.snapshot!.record!.id).toBe(fields.node.id);
      surface.unsubscribeNode(first); if (!late) throw new Error("Expected reentrant subscriber"); surface.unsubscribeNode(late); close(surface);
    });

    it("OwnedSurface rejects text and patch quotas without publishing and preserves rejected input ownership", async () => {
      const { OwnedUiSurface } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🖼️surface/🟦️.ts");
      const { OwnedUiOperation } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/🟦️.ts");
      const { RetainedUiTypedCursor } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️.ts");
      const { default: fields } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️tests/🧫️fixtures/🏷️fields/🔣️.json"); const { encodePackValue } = await import("@semio-tech/framework-os");
      for (const limit of ["maxTextBytes", "maxPatchBytes", "maxPatchOps"] as const) {
        const surface = new OwnedUiSurface({ actor: "quota", instance: 1, surface: "window" }, { ...DEFAULT_UI_DOCUMENT_LIMITS, [limit]: 0 }, { usizeBits: 64 }); const previous = surface.view;
        const decoder = new RetainedUiTypedCursor(encodePackValue({ ...fields.node, component: { type: "text", value: "Ü" } }), "node"); expect(finish(decoder)).toBe("ready"); const payload = decoder.takeResult()!; const operation = OwnedUiOperation.upsert(payload); retire(payload.beginClose()); close(decoder); const patch = surface.beginPatch(0, 1);
        if (limit === "maxPatchOps") { expect(() => patch.pushOperation(operation)).toThrow(); expect(operation.terminalIsEmpty()).toBe(false); retire(operation.beginClose()); }
        else { patch.pushOperation(operation); expect(finish(patch)).toBe("rejected"); }
        expect(surface.view).toBe(previous); expect(patch.takeAcknowledgement()).toBeNull(); close(patch); close(surface);
      }
    });

    it("OwnedSurface every cancellation prefix preserves bytes and completes committed notifications and receipt obligations", async () => {
      const { OwnedUiSurface } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🖼️surface/🟦️.ts");
      const { OwnedUiOperation } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/🟦️.ts");
      const { RetainedUiTypedCursor } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️.ts");
      const { default: fields } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️tests/🧫️fixtures/🏷️fields/🔣️.json"); const { encodePackValue } = await import("@semio-tech/framework-os");
      const run = (cursor: { advance(budget: typeof grant): { kind: string; bytes: number; items: number } }) => { for (let n = 0; n < 10000; n++) { const step = cursor.advance(grant); if (step.bytes > 4096 || step.items > 1) throw new Error("Exceeded grant"); if (step.kind === "ready") return; if (step.kind === "rejected") throw new Error("Unexpected rejection"); } throw new Error("Cursor failed to finish"); };
      const setup = () => {
        const surface = new OwnedUiSurface({ actor: "prefix", instance: 1, surface: "window" }, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 });
        const make = (byte: number) => { const decoder = new RetainedUiTypedCursor(encodePackValue({ ...fields.node, component: { ...fields.node.component, doc: { bytes: [byte,7,11] } } }), "node"); run(decoder); const payload = decoder.takeResult()!; const component = payload.value.component; if (component.type !== "surface") throw new Error("Expected bytes"); const operation = OwnedUiOperation.upsert(payload); retire(payload.beginClose()); close(decoder); return { operation, bytes: component.doc.bytes }; };
        const original = make(3); const seed = surface.beginPatch(0, 1); seed.pushOperation(original.operation); run(seed); seed.pushOperation(OwnedUiOperation.setRoot(fields.node.id)); run(seed); seed.finishInput(); run(seed); seed.takeAcknowledgement(); close(seed);
        const notices: number[] = []; const subscription = surface.subscribeNode(fields.node.id, () => notices.push(surface.view.revision)); while (surface.maintenancePending) surface.advanceMaintenance(grant);
        const candidate = make(23); const patch = surface.beginPatch(1, 2); patch.pushOperation(candidate.operation);
        return { surface, patch, subscription, notices, original: original.bytes, candidate: candidate.bytes };
      };
      let steps = 0; const phases = new Set<string>();
      const reference = setup(); let finishedInput = false;
      for (;;) { steps++; const step = reference.patch.advance(grant); phases.add(reference.patch.phase); if (step.kind === "rejected") throw new Error(reference.patch.failure!); if (step.kind === "ready") { if (!finishedInput) { reference.patch.finishInput(); finishedInput = true; } else break; } }
      reference.patch.takeAcknowledgement(); close(reference.patch); reference.surface.unsubscribeNode(reference.subscription); close(reference.surface);
      expect(phases).toEqual(new Set(["input", "validation", "scenes", "hash", "staging", "notifications", "accepted"]));
      for (let cutoff = 0; cutoff <= steps; cutoff++) {
        const owner = setup(); let inputDone = false;
        for (let n = 0; n < cutoff; n++) { const step = owner.patch.advance(grant); if (step.kind === "ready" && !inputDone) { owner.patch.finishInput(); inputDone = true; } }
        const committed = owner.surface.view.revision === 2; owner.patch.beginClose(); expect(owner.patch.closeStep({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); let receipts = 0;
        for (let n = 0; n < 20000; n++) { const step = owner.patch.closeStep(grant); if (step.bytes > 4096 || step.items > 1) throw new Error("Close exceeded grant"); if (step.kind === "blocked") { expect(step.phase).toBe("surface-acknowledgement"); expect(owner.patch.takeAcknowledgement()?.revision).toBe(2); receipts++; } if (step.kind === "complete") break; if (n === 19999) throw new Error("Close failed to finish"); }
        expect(owner.patch.terminalIsEmpty()).toBe(true); expect(receipts).toBe(committed ? 1 : 0); expect(owner.surface.view.revision).toBe(committed ? 2 : 1); expect(owner.notices).toEqual(committed ? [1,2] : [1]); expect(owner.original.byteAt(0)).toBe(3);
        if (committed) expect(owner.candidate.byteAt(0)).toBe(23); else expect(() => owner.candidate.byteAt(0)).toThrow();
        owner.surface.unsubscribeNode(owner.subscription); close(owner.surface); expect(() => owner.original.byteAt(0)).toThrow(); expect(() => owner.candidate.byteAt(0)).toThrow();
      }
      expect(steps).toBeGreaterThan(100);
    });

    it("ReadLease aborted speculative rendering never creates a subscription owner", async () => {
      const { useOwnedUiNode } = await import("../../📖️owned/🟦️.tsx"); const { render, cleanup } = await import("@testing-library/react"); const { Suspense, createElement } = await import("react");
      const subscribe = vi.fn(() => { throw new Error("Aborted render must not acquire a read lease"); });
      const source: import("../../📖️owned/🟦️.tsx").OwnedUiReadSource = { subscribeNode: subscribe, acknowledgeRead: vi.fn(), unsubscribeNode: vi.fn() };
      const suspended = new Promise<void>(() => {});
      function View(): never { useOwnedUiNode(source, 7); throw suspended; }
      const root = render(createElement(Suspense, { fallback: createElement("span", null, "Warten") }, createElement(View)));
      expect(root.getByText("Warten")).toBeTruthy(); expect(subscribe).not.toHaveBeenCalled(); root.unmount(); expect(source.unsubscribeNode).not.toHaveBeenCalled(); cleanup();
    });

    it("closes cancellation at every normalization step and traverses deep UiValue without recursive execution", async () => {
      const { RetainedUiTypedCursor } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️.ts");
      const { encodePackValue } = await import("@semio-tech/framework-os");
      const wire = { type: "surface", kind: "node-graph", docSchema: "node-graph@1", doc: { bytes: [2, 4] }, bindings: [{ trigger: "activate", action: { scope: "a", name: "b", version: 1 }, args: { nested: ["Ü", true] } }] };
      let normalizationSteps = 0;
      const reference = new RetainedUiTypedCursor(encodePackValue(wire), "component");
      for (let n = 0; n < 10000; n++) { const current = reference.advance(grant); if (current.phase === "typed-normalize") normalizationSteps++; if (current.kind === "ready") break; }
      close(reference); expect(normalizationSteps).toBeGreaterThan(20);
      for (let cutoff = -1; cutoff <= normalizationSteps; cutoff++) {
        const cursor = new RetainedUiTypedCursor(encodePackValue(wire), "component"); let observed = 0;
        if (cutoff >= 0) for (let n = 0; n < 10000; n++) { const current = cursor.advance(grant); if (current.phase === "typed-normalize" && observed++ === cutoff) break; if (current.kind === "ready") break; }
        cursor.beginClose(); expect(cursor.takeResult()).toBeNull(); expect(cursor.closeStep({ maxItems: 1, maxBytes: 4095 }).kind).toBe("blocked"); close(cursor);
      }
      let nested: unknown = "last"; for (let i = 0; i < 500; i++) nested = { next: nested };
      const deep = new RetainedUiTypedCursor(encodePackValue({ type: "extension", extension: "deep", props: nested }), "component"); expect(finish(deep), deep.failure ?? "").toBe("ready"); const payload = deep.takeResult()!; close(deep); expect(payload.value.type).toBe("extension"); retire(payload.beginClose());
    });
  });
  //#endregion 🧾️TypedWireTests
}
