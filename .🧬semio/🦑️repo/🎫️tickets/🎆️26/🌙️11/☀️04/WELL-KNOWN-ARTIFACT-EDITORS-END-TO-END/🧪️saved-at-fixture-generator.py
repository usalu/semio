import json, sys
from datetime import datetime, timezone
stamps = ["2026-11-04T06:41:07Z", "2026-11-04T06:41:07.5Z", "1969-12-31T23:59:59Z", "2000-02-29T12:00:00.123456789Z", "1970-01-01T00:00:00Z", "2100-03-01T00:00:00Z"]
rows = []
for stamp in stamps:
    value = datetime.fromisoformat(stamp.replace("Z", "+00:00")).astimezone(timezone.utc)
    seconds = int(value.timestamp()) if value.microsecond == 0 else None
    whole = int((value - datetime(1970, 1, 1, tzinfo=timezone.utc)).total_seconds() // 1)
    fraction = stamp.split(".")[1].rstrip("Z") if "." in stamp else ""
    nanos = whole * 10**9 + int((fraction + "0" * 9)[:9] or 0)
    rows.append({"stamp": stamp, "nanoseconds": str(nanos)})
rows += [{"stamp": "0", "nanoseconds": "0"}, {"stamp": "1730702467000", "nanoseconds": str(1730702467000 * 10**6)}]
refused = ["", "yesterday", "2026-11-04 06:41:07", "2026-13-04T06:41:07Z", "2026-11-04T06:41:07+01:00", "2026-11-04T06:41:07.Z"]
json.dump({"accepted": rows, "refused": refused}, sys.stdout, indent=1)
