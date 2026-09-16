# Shooting icon camera projection parity

## Root cause

- Scene window publishes `camera.projection` (`perspective` / `orthographic`) in world-3d JSON.
- `shooting_icon_render_request_json` omitted `projection`; icon host always built a `PerspectiveCamera`.
- `iconRenderCameraPose` fit lane used perspective FOV distance math only, so orthographic scene views (zoom-driven framing) produced overzoomed / misaligned icon shots when `center_model` was on.

## Fix

- Pass `projection` in shooting icon render requests.
- Extend `IconRenderCamera` with optional `projection`.
- `buildIconCamera` uses `OrthographicCamera` with shot-sized frustum when orthographic.
- `iconRenderCameraPose` fits orthographic shots by recomputing `zoom` from bounding sphere and shot aspect (drei-style frustum).
