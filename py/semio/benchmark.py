import time
import json
import os
from semio import Kit, validateKit, flattenDesignDict, _applyDesignDiff, Type, Design

ASSETS_DIR = "../../assets/semio"
ITERATIONS = 100

def load_kit(filename: str) -> dict:
    path = os.path.join(ASSETS_DIR, filename)
    if not os.path.exists(path):
        raise FileNotFoundError(f"Asset not found: {path}")
    with open(path, "r", encoding="utf-8") as f:
        data = json.load(f)
        if "guid" in data and "uri" not in data:
            data["uri"] = data["guid"]
        for key in ["types", "designs", "files", "folders", "authors", "concepts", "models", "connectors", "pieces", "connections", "layers", "groups", "stats", "ports", "qualities", "attributes"]:
             if key not in data or data[key] is None:
                 data[key] = []
        
        # Cleanup references to IDs
        for collection in ["types", "designs", "folders"]:
            if collection in data:
                for item in data[collection]:
                    if "parent" in item and isinstance(item["parent"], dict) and "guid" in item["parent"]:
                        item["parent"] = item["parent"]["guid"]
                    if "folder" in item and isinstance(item["folder"], dict) and "guid" in item["folder"]:
                        item["folder"] = item["folder"]["guid"]

        if "types" in data:
            for t in data["types"]:
                if "models" in t:
                    for m in t["models"]:
                        if "file" in m and isinstance(m["file"], dict) and "guid" in m["file"]:
                            m["file"] = m["file"]["guid"]

        return data

def bench(name: str, func):
    start = time.perf_counter()
    for _ in range(ITERATIONS):
        func()
    end = time.perf_counter()
    duration = (end - start) / ITERATIONS
    print(f"{name},{duration:.6f}")

def find_design(kit: dict, name: str, parent_name: str = None) -> dict:
    parent_guid = None
    if parent_name:
        for d in kit.get("designs", []):
            if d.get("name") == parent_name:
                parent_guid = d.get("guid")
                break
        if not parent_guid:
            raise ValueError(f"Parent {parent_name} not found")
            
    for d in kit.get("designs", []):
        if d.get("name") == name:
            p = d.get("parent")
            if parent_guid:
                if p and p.get("guid") == parent_guid:
                    return d
            else:
                if not p:
                    return d
    raise ValueError(f"Design {name} not found")

def main():
    kit_metabolism = load_kit("kit_metabolism.json")
    kit_invalid = load_kit("kit_invalid.json")
    
    kit_obj = Kit.model_validate(kit_metabolism)
    if kit_obj.types is None and "types" in kit_metabolism:
        kit_obj.types = [Type.model_validate(t) for t in kit_metabolism["types"]]
    
    kit_invalid_obj = Kit.model_validate(kit_invalid)
    if kit_invalid_obj.types is None and "types" in kit_invalid:
        kit_invalid_obj.types = [Type.model_validate(t) for t in kit_invalid["types"]]
    
    # 1. Roundtrip/Metabolism
    def test_roundtrip():
        s = kit_obj.model_dump_json()
        Kit.model_validate_json(s)
        
    bench("Roundtrip/Metabolism", test_roundtrip)
    
    # 2. Flatten Design/Nakagin Capsule Tower
    d1 = find_design(kit_metabolism, "Nakagin Capsule Tower")
    def test_flatten_nakagin():
        diff = flattenDesignDict(kit_metabolism, d1["guid"])
        _applyDesignDiff(d1, diff)
        
    bench("Flatten Design/Nakagin Capsule Tower", test_flatten_nakagin)
    
    # 3. Flatten Design/Nakagin Capsule Tower/Slanted
    d2 = find_design(kit_metabolism, "Slanted", "Nakagin Capsule Tower")
    def test_flatten_nakagin_slanted():
        diff = flattenDesignDict(kit_metabolism, d2["guid"])
        _applyDesignDiff(d2, diff)
        
    bench("Flatten Design/Nakagin Capsule Tower/Slanted", test_flatten_nakagin_slanted)

    # 4. Flatten Design/Nakagin Capsule Tower/Twisted
    d3 = find_design(kit_metabolism, "Twisted", "Nakagin Capsule Tower")
    def test_flatten_nakagin_twisted():
        diff = flattenDesignDict(kit_metabolism, d3["guid"])
        _applyDesignDiff(d3, diff)
        
    bench("Flatten Design/Nakagin Capsule Tower/Twisted", test_flatten_nakagin_twisted)

    # 5. Flatten Design/Nakagin Capsule Tower/Dancing
    d4 = find_design(kit_metabolism, "Dancing", "Nakagin Capsule Tower")
    def test_flatten_nakagin_dancing():
        diff = flattenDesignDict(kit_metabolism, d4["guid"])
        _applyDesignDiff(d4, diff)
        
    bench("Flatten Design/Nakagin Capsule Tower/Dancing", test_flatten_nakagin_dancing)

    # 6. Flatten Design/Capsule Dream
    d5 = find_design(kit_metabolism, "Capsule Dream")
    def test_flatten_capsule_dream():
        diff = flattenDesignDict(kit_metabolism, d5["guid"])
        _applyDesignDiff(d5, diff)
        
    bench("Flatten Design/Capsule Dream", test_flatten_capsule_dream)
    
    # 7. Validation/Invalid Kit
    def test_validate_invalid():
        validateKit(kit_invalid_obj)
        
    bench("Validation/Invalid Kit", test_validate_invalid)
    
    # 8. Validation/Metabolism
    def test_validate_metabolism():
        validateKit(kit_obj)
        
    bench("Validation/Metabolism", test_validate_metabolism)

if __name__ == "__main__":
    main()
