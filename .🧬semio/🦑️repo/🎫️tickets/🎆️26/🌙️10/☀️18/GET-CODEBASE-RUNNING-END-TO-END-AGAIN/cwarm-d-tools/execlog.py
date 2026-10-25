"""Parses a Bazel --execution_log_json_file (concatenated pretty JSON objects) into per-spawn rows: python3 execlog.py <log> [--json out]"""
import json, sys

def spawns(path):
    text = open(path, encoding="utf-8").read()
    decoder, index, rows = json.JSONDecoder(), 0, []
    while index < len(text):
        while index < len(text) and text[index].isspace():
            index += 1
        if index >= len(text):
            break
        row, index = decoder.raw_decode(text, index)
        rows.append(row)
    return rows

if __name__ == "__main__":
    for row in spawns(sys.argv[1]):
        print(row.get("targetLabel"), row.get("mnemonic"), "cacheable", row.get("cacheable"), "remotable", row.get("remotable"), "cacheHit", row.get("cacheHit"), "runner", row.get("runner"), "inputs", len(row.get("inputs", [])), "outputs", len(row.get("actualOutputs", [])))
