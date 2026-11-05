import json, psutil
needle = "retained_fill_pays_for_pixel_work_and_exposes_cancellation_boundaries"
rows = []
for process in psutil.process_iter(["pid", "ppid", "create_time", "cmdline"], ad_value=None):
    row = process.info
    command = row.get("cmdline") or []
    if needle in command and "cargo" in command:
        rows.append({"pid": row["pid"], "parent": row["ppid"], "birth": row["create_time"], "executable": command[0], "tail": command[-6:]})
print(json.dumps(rows))
