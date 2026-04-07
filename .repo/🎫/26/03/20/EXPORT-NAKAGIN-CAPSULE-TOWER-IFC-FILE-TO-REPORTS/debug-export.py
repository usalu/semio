#!/usr/bin/env python3

import json
import sys
import traceback
from pathlib import Path

# Add semio.py to path
semio_py_path = Path('/workspaces/semio/semio/py')
sys.path.insert(0, str(semio_py_path))

try:
    from semio import export_design_model
    print("✅ Import successful")
except ImportError as e:
    print(f"❌ Failed to import: {e}")
    sys.exit(1)

def debug_export():
    """Debug the IFC export process"""
    try:
        # Load the kit
        kit_path = Path('/workspaces/semio/semio/assets/semio/kit_metabolism.json')
        with open(kit_path, 'r', encoding='utf-8') as f:
            kit_data = json.load(f)
        
        # Find the design
        nakagin_designs = [
            design for design in kit_data.get('designs', [])
            if design.get('name') == 'Nakagin Capsule Tower'
        ]
        
        if not nakagin_designs:
            print('❌ Nakagin Capsule Tower design not found')
            return
        
        design = nakagin_designs[0]
        print(f"📝 Design: {design.get('name')}")
        print(f"🆔 GUID: {design.get('guid')}")
        print(f"🧩 Pieces: {len(design.get('pieces', []))}")
        
        # Check some sample data structure
        pieces = design.get('pieces', [])
        if pieces:
            piece = pieces[0]
            print(f"🔍 Sample piece: {piece.get('name')}")
            print(f"🔍 Piece type: {piece.get('type')}")
            
        # Try the export
        print("🔄 Attempting IFC export...")
        ifc_bytes = export_design_model(
            kit=kit_data,
            design_id=design.get('guid'),
            format='.ifc',
            tags=[],
            options={}
        )
        
        print(f"✅ Export successful! Size: {len(ifc_bytes)} bytes")
        
        # Save to test file
        output_path = Path('/workspaces/semio/.repo/🎫/26/03/20/EXPORT-NAKAGIN-CAPSULE-TOWER-IFC-FILE-TO-REPORTS/test-nakagin.ifc')
        with open(output_path, 'wb') as f:
            f.write(ifc_bytes)
        print(f"💾 Test file saved to: {output_path}")
        
    except Exception as e:
        print(f"❌ Error: {e}")
        print("📋 Full traceback:")
        traceback.print_exc()

if __name__ == '__main__':
    debug_export()
