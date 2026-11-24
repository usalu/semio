# 🌐️ Browser Check of the Staffage Site (2026-11-23 03:55–04:05)

Server: launch configuration `staffage-site-native` (vite prototype mode, port 6072). No console errors on any page below.

| Page | Seen |
| --- | --- |
| `#/` overview | 375 authored · 0 planned; humans 75, animals 14, vegetation 67, furniture 39, appliances 39, objects 58, vehicles 9, machinery 16, components 58; collections 23 |
| `#/humans/barista` (new wave human) | side elevation 1:100 with dimension 1.73 m, clip `latte-art-pour` playing; all seven representations, six viewpoints, four scales and custom scale offered |
| `#/humans/barista?vp=model` | 3D stage draws the volume model in dark line drawing and in thermal view (first paint takes a few seconds) |
| `#/humans/barista?view=thermal` | thermal side elevation with air flow streaks, legend and caption |
| `#/vegetation/sunflower` (PW-15 plant) | side elevation 1:100, 2.41 m, clip `wind-sway`; the per-asset check runs in the page |
| `#/collections` | 23 curated collections, first one drawn |
| `#/compare?p=barista~dark-line&p=barista~thermal` | two panes on one clock, playing |

Not looked at: playground, German language, light theme, mobile width, every other asset.

## Second Look after the Silhouette Repairs (2026-11-24 12:20)

Same server. The console held 119 `ERR_CONNECTION_REFUSED` lines from the stale tab before the server started; the count did not grow over two reloads and four page changes.

| Page | Seen |
| --- | --- |
| `#/` overview | loads, compare strip with walking commuter in line and thermal view |
| `#/machinery/tower-crane` | side elevation 1:500, lattice mast, jib, pendants and hook block drawn |
| `#/vegetation/bamboo-clump` | side elevation 1:100, 4.45 m, six culms with foliage; the per-asset check runs in the page |
| `#/vegetation/weeping-willow` (9.5 MB file) | side elevation 1:200 drawn within the 10 s wait |
| `#/humans/runner-with-blade-prosthesis?vp=model&view=thermal` | 3D stage draws the thermal volume model with flow streaks |
| `#/playground` | loads, asset list filling (96 of 375 when looked at), nothing placed |

Not looked at: German language, light theme, mobile width, placing figures in the playground.
