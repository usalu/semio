filepath = '/workspaces/semio/semio/js/sketchpad.test.ts'
with open(filepath, 'r') as f:
    content = f.read()

old_text = '''        const childPieceResult = await page.evaluate(() => {
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

new_text = '''        const childPieceGuid = await page.evaluate(() => {
          const store = (window as any).__SEMIO_STORE__;
          if (!store) return null;
          const kitGuids = Array.from((store as any).kits?.keys() ?? []) as string[];
          if (kitGuids.length === 0) return null;
          const kitStore = (store as any).kit(kitGuids[0]);
          if (!kitStore) return null;
          const kit = kitStore.snapshot();
          const url = window.location.pathname;
          const designGuidMatch = url.match(/\\/designs\\/([^/]+)/);
          const designGuid = designGuidMatch?.[1];
          const design = designGuid ? kit.designs?.find((d: any) => d.guid === designGuid) : kit.designs?.[kit.designs.length - 1];
          if (!design) return null;
          const connections = design.connections ?? [];
          const pieces = design.pieces ?? [];
          if (connections.length === 0 || pieces.length === 0) return null;
          const connectingGuids = new Set(connections.map((c: any) => c.connecting?.piece?.guid));
          const childPiece = pieces.find((p: any) => connectingGuids.has(p.guid));
          return childPiece?.guid || null;
        });
        console.log("[Design] Found child piece with parent connection:", childPieceGuid);'''

if old_text in content:
    content = content.replace(old_text, new_text, 1)
    with open(filepath, 'w') as f:
        f.write(content)
    print("SUCCESS: Fixed child piece search to use __SEMIO_STORE__")
else:
    print("ERROR: old text not found")
    idx = content.find('childPieceResult')
    if idx >= 0:
        print(f"Found childPieceResult at char {idx}")
    else:
        print("childPieceResult not found")
