filepath = '/workspaces/semio/semio/js/sketchpad.test.ts'
with open(filepath, 'r') as f:
    content = f.read()

old_text = '''        const guidApplied = await applySelectionShape("guid");
        console.log("[Design] Applied guid selection shape:", guidApplied);
        expect(guidApplied.applied).toBe(true);
        await validatePieceDetails("guid shape", true);

        const nodeIdApplied = await applySelectionShape("nodeId");'''

new_text = '''        const guidApplied = await applySelectionShape("guid");
        console.log("[Design] Applied guid selection shape:", guidApplied);
        expect(guidApplied.applied).toBe(true);
        await validatePieceDetails("guid shape", true);

        const childPieceGuid = await page.evaluate(() => {
          const actor = (window as any).__SEMIO_ACTOR__;
          if (!actor) return null;
          const snapshot = actor.getSnapshot();
          const path = window.location.pathname;
          const designGuidMatch = path.match(/\\/designs\\/([^/]+)/);
          const designGuid = designGuidMatch?.[1];
          const designApps = snapshot?.context?.designApps || {};
          const designAppKey = Object.keys(designApps).find((key: string) => key.endsWith(`:${designGuid}`) || key === designGuid) || "";
          const kitGuid = designAppKey.includes(":") ? designAppKey.split(":")[0] : Object.keys(snapshot?.context?.kits || {})[0];
          const kit = snapshot?.context?.kits?.[kitGuid];
          if (!kit || !designGuid) return null;
          const design = kit.designs?.find((d: any) => d.guid === designGuid);
          if (!design) return null;
          const connectingGuids = new Set((design.connections || []).map((c: any) => c.connecting?.piece?.guid));
          const childPiece = (design.pieces || []).find((p: any) => connectingGuids.has(p.guid));
          return childPiece?.guid || null;
        });
        console.log("[Design] Found child piece with parent connection:", childPieceGuid);
        if (childPieceGuid) {
          const childApplied = await page.evaluate(({ pieceGuid }: { pieceGuid: string }) => {
            const actor = (window as any).__SEMIO_ACTOR__;
            if (!actor) return { applied: false, reason: "missing-actor" };
            const snapshot = actor.getSnapshot();
            const path = window.location.pathname;
            const designGuidMatch = path.match(/\\/designs\\/([^/]+)/);
            const designGuid = designGuidMatch?.[1];
            const designApps = snapshot?.context?.designApps || {};
            const designAppKey = Object.keys(designApps).find((key: string) => key.endsWith(`:${designGuid}`) || key === designGuid) || "";
            const kitGuid = designAppKey.includes(":") ? designAppKey.split(":")[0] : Object.keys(snapshot?.context?.kits || {})[0];
            if (!designGuid || !kitGuid) return { applied: false, reason: "missing-scope" };
            actor.send({ type: "DESIGN.SET_SELECTION", kitGuid, designGuid, selection: { pieces: [pieceGuid], connections: [], connectors: [] } });
            return { applied: true };
          }, { pieceGuid: childPieceGuid });
          console.log("[Design] Applied child piece selection:", childApplied);
          expect(childApplied.applied).toBe(true);
          await validatePieceDetails("child piece with parent connection", true);
        }

        const nodeIdApplied = await applySelectionShape("nodeId");'''

if old_text in content:
    content = content.replace(old_text, new_text, 1)
    with open(filepath, 'w') as f:
        f.write(content)
    print("SUCCESS: Added child piece selection test on disk")
else:
    print("ERROR: Could not find target text on disk")
    idx = content.find('await validatePieceDetails("guid shape", true);')
    if idx >= 0:
        print(f"Found guid shape call at char {idx}")
        print("Context after:", repr(content[idx:idx+300]))
    else:
        print("guid shape call not found at all")
