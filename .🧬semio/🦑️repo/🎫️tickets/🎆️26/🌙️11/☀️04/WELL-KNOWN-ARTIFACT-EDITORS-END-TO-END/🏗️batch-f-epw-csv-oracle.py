import csv, io, json, sys

def main(epw_path, out_path):
    with open(epw_path, newline="", encoding="utf-8") as handle:
        text = handle.read()
    lines = text.split("\r\n")
    if lines[-1] == "":
        lines.pop()
    header = lines[:8]
    rows = [row for row in csv.reader(io.StringIO("\n".join(lines[8:]))) if row]
    assert all(len(row) == 35 for row in rows), "every EPW data row has 35 columns"
    json.dump({"schema": "semio.stdio.epw.csv-oracle.v1", "oracle": "python-stdlib-csv", "headerLines": header, "records": rows}, open(out_path, "w", encoding="utf-8"), ensure_ascii=False, indent=2)
    open(out_path, "a").write("\n")

main(sys.argv[1], sys.argv[2])
