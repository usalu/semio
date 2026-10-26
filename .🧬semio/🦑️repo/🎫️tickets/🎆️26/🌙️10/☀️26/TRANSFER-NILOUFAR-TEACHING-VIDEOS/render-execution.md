# Cooling and Energy Balance Rendering

Both series run through `repo run.ts` with the isolated generated Python environment and `🎓️teaching/render_videos.py`. Each invocation selects one series, uses `--quality l --no-play`, and places output under the ticket generated `videos` directory. These are complete authored scene timelines at 854×480 and 15 fps.

The cooling invocation covers seven scenes; energy balance covers eight. Both intro scenes completed and both series continued into their authored scenes without errors. Final completion and media validation are pending while the render children run.

Runtime logs are generated files and will be removed with the ticket generated directory. The transferred source snapshot and its SHA-256 manifest remain documented in `transfer.md`.

Monitoring was handed to the portability scheduler while both original render runners continue. Cooling session: `77176`; Python process `25824`, parent redirector `9864`, `run.ts` parent `9996`. Energy balance session: `45453`; Python process `41452`, parent redirector `2988`, `run.ts` parent `2708`. Latest observed active classes were `Cooling_03_TransmissionHumidity` and `EnergyBalance_01_Energiebilanz`. The ticket log watcher was stopped after handoff; no render was interrupted.

The launcher is `🗑️generated/launcher/repo.exe`. Set `SEMIO_RUNTIME_CACHE` to this ticket's `🗑️generated/runtimes` and select `SEMIO_LANGUAGE=en` for launcher commands. Render commands use `PYTHONUTF8=1`, `PYTHONIOENCODING=utf-8`, and should use `PYTHONDONTWRITEBYTECODE=1`. The isolated environment also suppresses bytecode through an early `.pth` hook and `sitecustomize.py`; `prevent-cache.ts` preserves this setup procedure. See `path-audit.md` for the latest complete generated-path scan.
