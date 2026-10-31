# Native Memory Observations

The completed standalone profile did not measure OS peak memory. Native runtime memory observations are separate, finite samples from the official capacity/process interfaces. At09:28:27UTC the root Grasshopper command reported that Windows process-tree footprint collection failed and resident set was used instead, explicitly warning that it can overstate holdings. The readings below must not be promoted to exact private footprint or OS peak measurements. A zero cpuPercent field in the selected Windows doctor snapshot is not proof of a hang or progress.

All three owned runtimes were authoritatively live at the latest sample. Another logical owners jobs were also admitted/waiting; no unowned process was signalled. Normal pressure and current spare admission are observations, not a future resource guarantee. No performance SLO was supplied or claimed. Root keeps exact native results/current dependencies mandatory and records actual limitations.

The child log hash below identifies only the prefix read during this observation; the live log may append. If captured_warning_lines is empty, the warning was visible on the outer command stream rather than this child log.

```json
{
  "observed_at": "2026-10-30T09:31:46.493181+00:00",
  "current_owned_roots": {
    "capture": 12384,
    "grasshopper": 40200,
    "excel_replacement": 42548
  },
  "latest_capacity_observation": {
    "available_MB": 10570,
    "floor_MB": 4096,
    "reported_spare_MB": 1289,
    "pressure": "normal",
    "grasshopper_observed_MB": 26142,
    "excel_observed_MB": 3270,
    "capture_observed_MB": 1917
  },
  "ongoing_child_log_observed_prefix_bytes": 18684,
  "ongoing_child_log_observed_prefix_sha256": "9eef73e98bf9f88668124600c9cfdd06fc3f7dd5f965a24e7f8343b863194b23",
  "captured_warning_lines": []
}
```
