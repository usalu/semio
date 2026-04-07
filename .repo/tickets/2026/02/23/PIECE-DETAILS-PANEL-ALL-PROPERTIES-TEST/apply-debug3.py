import re

filepath = "/workspaces/semio/semio/js/sketchpad.test.ts"
with open(filepath, "r") as f:
    content = f.read()

old = '''          const childApplied = await page.evaluate(({ pieceGuid }: { pieceGuid: string }) => {
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
            actor.send({ type: "DESIGN.SET_ACTIVE_TOOL", kitGuid, designGuid, activeTool: "selection-normal" });
            actor.send({ type: "DESIGN.SET_SELECTION", kitGuid, designGuid, selection: { pieces: [pieceGuid], connections: [], connectors: [] } });
            return { applied: true };
          }, { pieceGuid: childPieceGuid });
          console.log("[Design] Applied child piece selection:", childApplied);
          expect(childApplied.applied).toBe(true);
          await validatePieceDetails("child piece with parent connection", true);'''

new = '''          const childApplied = await page.evaluate(({ pieceGuid }: { pieceGuid: string }) => {
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
            actor.send({ type: "DESIGN.SET_ACTIVE_TOOL", kitGuid, designGuid, activeTool: "selection-normal" });
            actor.send({ type: "DESIGN.SET_SELECTION", kitGuid, designGuid, selection: { pieces: [pieceGuid], connections: [], connectors: [] } });
            const afterSnapshot = actor.getSnapshot();
            const afterDesignApp = afterSnapshot?.context?.designApps?.[designAppKey];
            return { applied: true, kitGuid, designGuid, designAppKey, selection: afterDesignApp?.selection, activeTool: afterDesignApp?.activeTool };
          }, { pieceGuid: childPieceGuid });
          console.log("[Design] Applied child piece selection:", JSON.stringify(childApplied));
          expect(childApplied.applied).toBe(true);
          await page.waitForTimeout(2000);
          const panelHtml = await page.locator('[data-panel="rightSidePanel"]').first().innerHTML().catch(() => "PANEL_NOT_FOUND");
          console.log("[DEBUG] Right panel HTML length after child selection:", panelHtml.length);
          console.log("[DEBUG] Right panel contains piece section:", panelHtml.includes("section.piece.properties"));
          console.log("[DEBUG] Right panel contains piece id:", panelHtml.includes("piece.id"));
          console.log("[DEBUG] Right panel first 500 chars:", panelHtml.substring(0, 500));
          await validatePieceDetails("child piece with parent connection", true);'''

if old in content:
    content = content.replace(old, new)
    with open(filepath, "w") as f:
        f.write(content)
    print("SUCCESS: Added debug output for child selection")
else:
    print("ERROR: Old string not found")
    # Try to find nearby content
    idx = content.find("childApplied")
    if idx >= 0:
        print(f"Found 'childApplied' at index {idx}")
        print("Context:", content[idx-100:idx+200])
