#!/usr/bin/env python3

# Test script to verify IFC export functionality for reports

import json
import sys
from pathlib import Path

# Add compose.py to path
compose_py_path = Path('/workspaces/semio/compose/py')
sys.path.insert(0, str(compose_py_path))

def test_ifc_export():
    """Test IFC export and verify file properties"""
    print("🧪 Testing Nakagin Capsule Tower IFC export for reports...")
    
    # Check if IFC file exists
    ifc_path = Path('/workspaces/semio/assets/models/nakagin-capsule-tower.ifc')
    
    if not ifc_path.exists():
        print("❌ IFC file not found. Run export-to-ifc.py first.")
        return False
    
    # Check file size
    file_size = ifc_path.stat().st_size
    print(f"📊 IFC file size: {file_size:,} bytes")
    
    # Check file content (basic validation)
    try:
        with open(ifc_path, 'r', encoding='utf-8') as f:
            first_line = f.readline().strip()
            if first_line == "ISO-10303-21;":
                print("✅ Valid IFC file format detected")
            else:
                print(f"❌ Invalid IFC format: {first_line}")
                return False
    except Exception as e:
        print(f"❌ Error reading IFC file: {e}")
        return False
    
    # Check if we can read basic IFC content
    try:
        content = ifc_path.read_text(encoding='utf-8')
        if "IFCPROJECT" in content and "IFC4" in content:
            print("✅ IFC file contains expected IFC4 schema elements")
        else:
            print("❌ IFC file missing expected schema elements")
            return False
    except Exception as e:
        print(f"❌ Error parsing IFC content: {e}")
        return False
    
    # Compare with existing GLB export if available
    glb_path = Path('/workspaces/semio/assets/models/nakagin-capsule-tower.glb')
    if glb_path.exists():
        glb_size = glb_path.stat().st_size
        print(f"📊 GLB file size: {glb_size:,} bytes")
        print(f"📈 IFC/GLB size ratio: {file_size/glb_size:.2f}")
    
    print("✅ IFC export test completed successfully!")
    return True

if __name__ == '__main__':
    success = test_ifc_export()
    sys.exit(0 if success else 1)
