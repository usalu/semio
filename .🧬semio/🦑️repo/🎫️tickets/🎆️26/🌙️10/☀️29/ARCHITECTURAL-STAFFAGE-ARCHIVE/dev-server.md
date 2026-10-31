# Staffage dev server

The site is `@teaching/staffage-site`. Its fixed port is 6071 (`TEACHING_ARCHITECTURE_STAFFAGE_PORT`).

The process that was already listening bound only `[::1]:6071`, so `http://127.0.0.1:6071/` refused the connection while `http://localhost:6071/` worked over IPv6. That process was stopped and Vite was started again with `--host ::`, which listens on `*:6071`.

Verified after the restart: `http://127.0.0.1:6071/`, `http://[::1]:6071/` and `http://localhost:6071/` all return 200, and the document title is `Architectural Staffage Archive · Archiv architektonischer Staffage`. Before the restart, a browser load of the overview showed all 48 assets with no console errors, and these routes loaded without page errors: oak detail, thermal view, forces view, compare, playground, collections, vehicles, humans.

Open: http://localhost:6071/
