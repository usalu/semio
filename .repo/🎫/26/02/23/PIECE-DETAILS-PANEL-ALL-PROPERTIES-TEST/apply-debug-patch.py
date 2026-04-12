filepath = '/workspaces/semio/semio/js/sketchpad.test.ts'
with open(filepath, 'r') as f:
    content = f.read()

old_text = '''        const childPieceGuid = await page.evaluate(() => {
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
        });'''

new_text = '''        const childPieceGuid = await page.evaluate(() => {
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
          console.log("[DEBUG] childPiece search: kitGuid=", kitGuid, "designGuid=", designGuid, "hasKit=", !!kit, "kitKeys=", kit ? Object.keys(kit).join(",") : "none");
          if (!kit || !designGuid) return null;
          const designs = kit.designs || kit.design ? [kit.design] : [];
          const design = Array.isArray(designs) ? designs.find((d: any) => d.guid === designGuid) : null;
          const pieces = design?.pieces || kit.pieces || [];
          const connections = design?.connections || kit.connections || [];
          console.log("[DEBUG] childPiece search: designsCount=", Array.isArray(designs) ? designs.length : "not-array", "pieces=", pieces.length, "connections=", connections.length);
          if (connections.length === 0 || pieces.length === 0) return null;
          const connectingGuids = new Set(connections.map((c: any) => c.connecting?.piece?.guid));
          const childPiece = pieces.find((p: any) => connectingGuids.has(p.guid));
          console.log("[DEBUG] childPiece search: connectingGuidsSize=", connectingGuids.size, "found=", !!childPiece, "guid=", childPiece?.guid);
          return childPiece?.guid || null;
        });'''

if old_text in content:
    content = content.replace(old_text, new_text, 1)
    with open(filepath, 'w') as f:
        f.write(content)
    print("SUCCESS: Added debug logging to child piece search")
else:
    print("ERROR: Could not find target text")
