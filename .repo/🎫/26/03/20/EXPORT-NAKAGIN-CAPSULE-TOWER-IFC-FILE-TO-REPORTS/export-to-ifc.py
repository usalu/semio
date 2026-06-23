#!/usr/bin/env python3

# region 🔖Header
# Export Nakagin Capsule Tower Design Model to IFC
# 
# This script extracts the Nakagin Capsule Tower design from the Metabolism kit
# and exports it as an IFC (Industry Foundation Classes) 3D model file.
# endregion 🔖Header

import json
import sys
import os
from pathlib import Path

# Add compose.py to path
compose_py_path = Path('/workspaces/semio/compose/py')
sys.path.insert(0, str(compose_py_path))

try:
    from compose import export_design_model
except ImportError as e:
    print(f"❌ Failed to import export_design_model from compose.py: {e}")
    print(f"❌ Tried to import from: {compose_py_path}")
    sys.exit(1)

# region 🔖Main
# Main export logic for IFC format

def extract_nakagin_capsule_tower_design():
    """Extract Nakagin Capsule Tower design from Metabolism kit"""
    try:
        kit_path = Path('/workspaces/semio/compose/assets/compose/kit_metabolism.json')
        
        if not kit_path.exists():
            print(f"❌ Metabolism kit file not found: {kit_path}")
            return None, None
            
        with open(kit_path, 'r', encoding='utf-8') as f:
            kit_data = json.load(f)
        
        nakagin_designs = [
            design for design in kit_data.get('designs', [])
            if design.get('name') == 'Nakagin Capsule Tower'
        ]
        
        if not nakagin_designs:
            print('❌ Nakagin Capsule Tower design not found in Metabolism kit')
            return None, None
        
        print(f"✅ Found {len(nakagin_designs)} Nakagin Capsule Tower design(s)")
        return kit_data, nakagin_designs[0]  # Return kit and main design
        
    except Exception as error:
        print(f"❌ Failed to read Metabolism kit: {error}")
        return None, None

def export_ifc_model(kit, design, output_path):
    """Export design to IFC format using compose.py export_design_model"""
    try:
        print(f"🔄 Exporting {design['name']} to IFC format...")
        
        # Use the export_design_model function from compose.py
        ifc_bytes = export_design_model(
            kit=kit,
            design_id=design.get('guid'),
            format='.ifc',
            tags=[],  # No specific tags for now
            options={}  # Default options
        )
        
        # Ensure output directory exists
        output_path.parent.mkdir(parents=True, exist_ok=True)
        
        # Write IFC bytes to file
        with open(output_path, 'wb') as f:
            f.write(ifc_bytes)
        
        print(f"✅ IFC exported to: {output_path}")
        print(f"📊 Model size: {len(ifc_bytes)} bytes")
        
        return True
        
    except Exception as error:
        print(f"❌ Failed to export IFC: {error}")
        import traceback
        traceback.print_exc()
        return False

def main():
    """Main execution function"""
    print('🚀 Starting Nakagin Capsule Tower design model export to IFC format...')
    
    # Extract the design
    kit, design = extract_nakagin_capsule_tower_design()
    if not kit or not design:
        sys.exit(1)
    
    print(f"📝 Design: {design.get('name')}")
    print(f"🆔 GUID: {design.get('guid')}")
    print(f"🧩 Pieces: {len(design.get('pieces', []))}")
    
    # Define output path
    output_dir = Path('/workspaces/semio/compose/assets/models')
    output_dir.mkdir(parents=True, exist_ok=True)
    output_path = output_dir / 'nakagin-capsule-tower.ifc'
    
    # Export the model
    success = export_ifc_model(kit, design, output_path)
    
    if success:
        print('🎉 IFC export completed successfully!')
        print(f'💡 IFC file saved to: {output_path}')
        print('🏗️  The IFC file can be opened in any IFC-compatible BIM software')
    else:
        print('❌ IFC export failed!')
        sys.exit(1)

if __name__ == '__main__':
    main()

# endregion 🔖Main
