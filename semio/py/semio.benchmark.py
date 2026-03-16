# region Header
# [👤semio📚py🥼semiobenchmark](semiorepo://p/u/semio/b/l/py/f/semio.benchmark.py)

# 2026 Ueli Saluz <ueli@semio-tech.de>

# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as
# published by the Free Software Foundation, either version 3 of the
# License, or (at your option) any later version.
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Affero General Public License for more details.
# You should have received a copy of the GNU Affero General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

# endregion Header

import json
import os
import time

from semio import Design, Kit, Type, _applyDesignDiff, applyKitDiffDict, flattenDesignDict, validateKit, validateKitDict

ASSETS_DIR = "../assets/semio"
ITERATIONS = 3


def load_json(filename: str) -> dict:
    path = os.path.join(ASSETS_DIR, filename)
    if not os.path.exists(path):
        raise FileNotFoundError(f"Asset not found: {path}")
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)


def load_kit(filename: str) -> dict:
    data = load_json(filename)
    if "guid" in data and "uri" not in data:
        data["uri"] = data["guid"]
    for key in ["types", "designs", "files", "folders", "authors", "concepts", "models", "connectors", "pieces", "connections", "layers", "groups", "stats", "ports", "qualities", "attributes"]:
        if key not in data or data[key] is None:
            data[key] = []

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
                    if "file" not in m or m["file"] is None:
                        m["file"] = ""

                    if "url" not in m or m["url"] is None:
                        m["url"] = ""

                    if "tags" in m and isinstance(m["tags"], list):
                        new_tags = []
                        for tag in m["tags"]:
                            if isinstance(tag, dict) and "guid" in tag:
                                new_tags.append(tag["guid"])
                            elif isinstance(tag, str):
                                new_tags.append(tag)
                        m["tags"] = new_tags
                    elif "tags" not in m:
                        m["tags"] = []

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
                if p:
                    p_guid = p.get("guid") if isinstance(p, dict) else p
                    if p_guid == parent_guid:
                        return d
            else:
                if not p:
                    return d
    raise ValueError(f"Design {name} not found")


def main():
    kit_metabolism = load_kit("kit_metabolism.json")
    kit_invalid = load_kit("kit_invalid.json")

    kit_obj = Kit.parse(kit_metabolism)

    kit_invalid_obj = Kit.parse(kit_invalid)

    def test_roundtrip():
        from semio import export_kit, import_kit

        kit, files = import_kit(os.path.join(ASSETS_DIR, "metabolism.zip"))

        export_kit(kit, files, "temp_benchmark_metabolism.zip")
        if os.path.exists("temp_benchmark_metabolism.zip"):
            os.remove("temp_benchmark_metabolism.zip")

    bench("Roundtrip/Metabolism", test_roundtrip)

    diff_forward = load_json("diff_kit_metabolism.json")
    diff_inverse = load_json("diff_kit_metabolism_inverted.json")

    def test_diff_metabolism():
        k2 = applyKitDiffDict(kit_metabolism, diff_forward)
        applyKitDiffDict(k2, diff_inverse)

    bench("Diff/Metabolism", test_diff_metabolism)

    d1 = find_design(kit_metabolism, "Nakagin Capsule Tower")

    def test_flatten_nakagin():
        flattenDesignDict(kit_metabolism, d1["guid"])

    bench("Flatten Design/Nakagin Capsule Tower", test_flatten_nakagin)

    d2 = find_design(kit_metabolism, "Slanted", "Nakagin Capsule Tower")

    def test_flatten_nakagin_slanted():
        flattenDesignDict(kit_metabolism, d2["guid"])

    bench("Flatten Design/Nakagin Capsule Tower/Slanted", test_flatten_nakagin_slanted)

    d3 = find_design(kit_metabolism, "Twisted", "Nakagin Capsule Tower")

    def test_flatten_nakagin_twisted():
        flattenDesignDict(kit_metabolism, d3["guid"])

    bench("Flatten Design/Nakagin Capsule Tower/Twisted", test_flatten_nakagin_twisted)

    d4 = find_design(kit_metabolism, "Dancing", "Nakagin Capsule Tower")

    def test_flatten_nakagin_dancing():
        flattenDesignDict(kit_metabolism, d4["guid"])

    bench("Flatten Design/Nakagin Capsule Tower/Dancing", test_flatten_nakagin_dancing)

    d5 = find_design(kit_metabolism, "Capsule Dream")

    def test_flatten_capsule_dream():
        flattenDesignDict(kit_metabolism, d5["guid"])

    bench("Flatten Design/Capsule Dream", test_flatten_capsule_dream)

    def test_validate_invalid():
        validateKit(kit_invalid_obj)

    bench("Validation/Invalid Kit", test_validate_invalid)

    def test_validate_metabolism():
        validateKit(kit_obj)

    bench("Validation/Metabolism", test_validate_metabolism)


if __name__ == "__main__":
    main()
