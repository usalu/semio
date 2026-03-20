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
        });
        console.log("[Design] Found child piece with parent connection:", childPieceGuid);'''

new_text = '''        const childPieceResult = await page.evaluate(() => {
          const actor = (window as any).__SEMIO_ACTOR__;
          if (!actor) return { guid: null, debug: "no-actor" };
          const snapshot = actor.getSnapshot();
          const path = window.location.pathname;
          const designGuidMatch = path.match(/\\/designs\\/([^/]+)/);
          const designGuid = designGuidMatch?.[1];
          const designApps = snapshot?.context?.designApps || {};
          const designAppKey = Object.keys(designApps).find((key: string) => key.endsWith(`:${designGuid}`) || key === designGuid) || "";
          const kitGuid = designAppKey.includes(":") ? designAppKey.split(":")[0] : Object.keys(snapshot?.context?.kits || {})[0];
          const kit = snapshot?.context?.kits?.[kitGuid];
          const kitKeys = kit ? Object.keys(kit).slice(0, 20).join(",") : "none";
          if (!kit || !designGuid) return { guid: null, debug: `no-kit-or-design kitGuid=${kitGuid} designGuid=${designGuid} kitKeys=${kitKeys}` };
          const designStore = kit._designStores ? Object.values(kit._designStores)[0] : null;
          const kitSnapshot = typeof kit.snapshot === "function" ? kit.snapshot() : kit;
          const designs = kitSnapshot.designs || [];
          const design = Array.isArray(designs) ? designs.find((d: any) => d.guid === designGuid) : null;
          const pieces = design?.pieces || [];
          const connections = design?.connections || [];
          if (connections.length === 0 || pieces.length === 0) return { guid: null, debug: `no-data kitKeys=${kitKeys} designsLen=${designs.length} designFound=${!!design} piecesLen=${pieces.length} connsLen=${connections.length}` };
          const connectingGuids = new Set(connections.map((c: any) => c.connecting?.piece?.guid));
          const childPiece = pieces.find((p: any) => connectingGuids.has(p.guid));
          return { guid: childPiece?.guid || null, debug: `found connectingGuids=${connectingGuids.size} childFound=${!!childPiece}` };
        });
        const childPieceGuid = childPieceResult?.guid || null;
        console.log("[Design] Child piece search result:", JSON.stringify(childPieceResult));'''

if old_text in content:
    content = content.replace(old_text, new_text, 1)
    with open(filepath, 'w') as f:
        f.write(content)
    print("SUCCESS")
else:
    print("ERROR: old text not found")
    # Try simpler check
    idx = content.find('const childPieceGuid = await page.evaluate')
    if idx >= 0:
        print(f"Found at char {idx}")
        print(repr(content[idx:idx+100]))
    else:
        # Check alternate line
        idx = content.find('childPieceGuid')
        if idx >= 0:
            print(f"childPieceGuid at char {idx}")
            print(repr(content[idx:idx+100]))
        else:
            print("No childPieceGuid found")
