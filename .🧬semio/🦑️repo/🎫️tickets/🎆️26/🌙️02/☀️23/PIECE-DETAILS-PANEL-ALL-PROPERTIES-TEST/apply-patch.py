import re

filepath = '/workspaces/semio/compose/js/sketchpad.test.ts'
with open(filepath, 'r') as f:
    content = f.read()

old_fn = '''        const validatePieceDetails = async (label: string) => {
          await page.waitForTimeout(700);
          await openDetailsPanel(page);
          const pieceSection = page.locator('[data-panel="rightSidePanel"] [id="compose.sketchpad.app.design.panel.details.section.piece.properties"]').first();
          const pieceIdInput = page.locator('[data-panel="rightSidePanel"] [id="compose.sketchpad.app.design.piece.id"]').first();
          const fallbackText = page.locator('[data-panel="rightSidePanel"] text=No valid pieces found in selection.').first();
          const hasPieceSection = await pieceSection.isVisible({ timeout: 5000 }).catch(() => false);
          const hasPieceIdInput = await pieceIdInput.isVisible({ timeout: 5000 }).catch(() => false);
          const hasFallback = await fallbackText.isVisible({ timeout: 1500 }).catch(() => false);
          console.log(`[Design] ${label} => pieceSection=${hasPieceSection}, pieceIdInput=${hasPieceIdInput}, fallback=${hasFallback}`);
          expect(hasPieceSection || hasPieceIdInput).toBe(true);
          expect(hasFallback).toBe(false);
        };

        const guidApplied = await applySelectionShape("guid");
        console.log("[Design] Applied guid selection shape:", guidApplied);
        expect(guidApplied.applied).toBe(true);
        await validatePieceDetails("guid shape");'''

new_fn = '''        const validatePieceDetails = async (label: string, checkAllProperties = false) => {
          await page.waitForTimeout(700);
          await openDetailsPanel(page);
          const panel = '[data-panel="rightSidePanel"]';
          const pieceSection = page.locator(`${panel} [id="compose.sketchpad.app.design.panel.details.section.piece.properties"]`).first();
          const pieceIdInput = page.locator(`${panel} [id="compose.sketchpad.app.design.piece.id"]`).first();
          const fallbackText = page.locator(`${panel} text=No valid pieces found in selection.`).first();
          const hasPieceSection = await pieceSection.isVisible({ timeout: 5000 }).catch(() => false);
          const hasPieceIdInput = await pieceIdInput.isVisible({ timeout: 5000 }).catch(() => false);
          const hasFallback = await fallbackText.isVisible({ timeout: 1500 }).catch(() => false);
          console.log(`[Design] ${label} => pieceSection=${hasPieceSection}, pieceIdInput=${hasPieceIdInput}, fallback=${hasFallback}`);
          expect(hasPieceSection || hasPieceIdInput).toBe(true);
          expect(hasFallback).toBe(false);
          if (checkAllProperties) {
            console.log(`[DEBUG] ${label} starting comprehensive piece property checks`);
            const checkVisible = async (id: string): Promise<boolean> => {
              const el = page.locator(`${panel} [id="${id}"]`).first();
              return el.isVisible({ timeout: 3000 }).catch(() => false);
            };
            const pieceName = await checkVisible("compose.sketchpad.app.design.panel.details.section.piece.name");
            const pieceType = await checkVisible("compose.sketchpad.app.design.piece.type");
            const pieceDescription = await checkVisible("compose.sketchpad.app.design.panel.details.section.piece.description");
            const pieceScale = await checkVisible("compose.sketchpad.app.design.panel.details.section.piece.scale");
            const pieceColor = await checkVisible("compose.sketchpad.app.design.panel.details.section.piece.color");
            console.log(`[Design] ${label} piece fields => name=${pieceName}, type=${pieceType}, description=${pieceDescription}, scale=${pieceScale}, color=${pieceColor}`);
            expect(pieceName).toBe(true);
            expect(pieceType).toBe(true);
            expect(pieceDescription).toBe(true);
            expect(pieceScale).toBe(true);
            expect(pieceColor).toBe(true);
            const hasParentConnection = await page.locator(`${panel} [id="compose.sketchpad.app.design.panel.details.section.connection.connecting"]`).first().isVisible({ timeout: 3000 }).catch(() => false);
            console.log(`[Design] ${label} hasParentConnection=${hasParentConnection}`);
            if (hasParentConnection) {
              const connectingPieceId = await checkVisible("compose.sketchpad.app.design.panel.details.section.connection.connectingPieceId");
              const connectingConnectorId = await checkVisible("compose.sketchpad.app.design.panel.details.section.connection.connectingConnectorId");
              const connectedPieceId = await checkVisible("compose.sketchpad.app.design.panel.details.section.connection.connectedPieceId");
              const connectedConnectorId = await checkVisible("compose.sketchpad.app.design.panel.details.section.connection.connectedConnectorId");
              console.log(`[Design] ${label} connection IDs => connectingPiece=${connectingPieceId}, connectingConnector=${connectingConnectorId}, connectedPiece=${connectedPieceId}, connectedConnector=${connectedConnectorId}`);
              expect(connectingPieceId).toBe(true);
              expect(connectingConnectorId).toBe(true);
              expect(connectedPieceId).toBe(true);
              expect(connectedConnectorId).toBe(true);
              const gap = await checkVisible("compose.sketchpad.app.design.panel.details.section.connection.gap");
              const shift = await checkVisible("compose.sketchpad.app.design.panel.details.section.connection.shift");
              const rise = await checkVisible("compose.sketchpad.app.design.panel.details.section.connection.rise");
              console.log(`[Design] ${label} translation => gap=${gap}, shift=${shift}, rise=${rise}`);
              expect(gap).toBe(true);
              expect(shift).toBe(true);
              expect(rise).toBe(true);
              const rotation = await checkVisible("compose.sketchpad.app.design.panel.details.section.connection.rotation");
              const turn = await checkVisible("compose.sketchpad.app.design.panel.details.section.connection.turn");
              const tilt = await checkVisible("compose.sketchpad.app.design.panel.details.section.connection.tilt");
              console.log(`[Design] ${label} orientation => rotation=${rotation}, turn=${turn}, tilt=${tilt}`);
              expect(rotation).toBe(true);
              expect(turn).toBe(true);
              expect(tilt).toBe(true);
              const u = await checkVisible("compose.sketchpad.app.design.panel.details.section.connection.u");
              const v = await checkVisible("compose.sketchpad.app.design.panel.details.section.connection.v");
              console.log(`[Design] ${label} diagram => u=${u}, v=${v}`);
              expect(u).toBe(true);
              expect(v).toBe(true);
            }
          }
        };

        const guidApplied = await applySelectionShape("guid");
        console.log("[Design] Applied guid selection shape:", guidApplied);
        expect(guidApplied.applied).toBe(true);
        await validatePieceDetails("guid shape", true);'''

if old_fn in content:
    content = content.replace(old_fn, new_fn, 1)
    with open(filepath, 'w') as f:
        f.write(content)
    print("SUCCESS: Replaced validatePieceDetails on disk")
else:
    print("ERROR: Could not find old function text on disk")
    # Try to find approximate location
    idx = content.find('const validatePieceDetails = async (label: string)')
    if idx >= 0:
        line_num = content[:idx].count('\n') + 1
        print(f"Found validatePieceDetails at line {line_num}")
        print("Context:", repr(content[idx:idx+200]))
    else:
        print("validatePieceDetails not found at all")
