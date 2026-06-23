filepath = '/workspaces/semio/compose/js/sketchpad.test.ts'
with open(filepath, 'r') as f:
    content = f.read()

old_text = '''            if (!designGuid || !kitGuid) return { applied: false, reason: "missing-scope" };
            actor.send({ type: "DESIGN.SET_SELECTION", kitGuid, designGuid, selection: { pieces: [pieceGuid], connections: [], connectors: [] } });
            return { applied: true };
          }, { pieceGuid: childPieceGuid });'''

new_text = '''            if (!designGuid || !kitGuid) return { applied: false, reason: "missing-scope" };
            actor.send({ type: "DESIGN.SET_ACTIVE_TOOL", kitGuid, designGuid, activeTool: "selection-normal" });
            actor.send({ type: "DESIGN.SET_SELECTION", kitGuid, designGuid, selection: { pieces: [pieceGuid], connections: [], connectors: [] } });
            return { applied: true };
          }, { pieceGuid: childPieceGuid });'''

if old_text in content:
    content = content.replace(old_text, new_text, 1)
    with open(filepath, 'w') as f:
        f.write(content)
    print("SUCCESS: Added SET_ACTIVE_TOOL to child piece selection")
else:
    print("ERROR: old text not found")
