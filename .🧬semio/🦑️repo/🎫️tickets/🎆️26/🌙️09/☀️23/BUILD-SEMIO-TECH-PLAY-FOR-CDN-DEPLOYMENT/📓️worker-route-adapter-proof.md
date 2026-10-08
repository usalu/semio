# Worker Route Adapter Proof

Ran 2026-10-07T00:35:13.161Z using the exact production `installPublishedPlayRoutes` installer and actual acceptance browser-launch settings. All pages in this probe are explicitly synthetic.

Status: PASSED.

The worker launched from the app’s local origin; fetches and the dynamic module import retained their exact baked HTTPS CDN origins and percent-encoded emoji paths in browser responses. The production adapter fulfilled them from the corresponding independent static satellite listeners.

Observed worker result:

```json
{
  "ok": true,
  "origin": "http://127.0.0.1:53998",
  "fetches": [
    {
      "url": "https://map.assets.semio-tech.com/osm/%F0%9F%A7%AA%EF%B8%8Frouting/%F0%9F%8C%8D%EF%B8%8Fmap.json",
      "status": 200,
      "token": "map-route-proof"
    },
    {
      "url": "https://media.assets.semio-tech.com/%F0%9F%96%BC%EF%B8%8Fassets/%F0%9F%A7%AA%EF%B8%8Frouting/%F0%9F%96%BC%EF%B8%8Fmedia.json",
      "status": 200,
      "token": "media-route-proof"
    },
    {
      "url": "https://modules.assets.semio-tech.com/%F0%9F%94%8C%EF%B8%8Fplugin-modules/%F0%9F%A7%AA%EF%B8%8Frouting/%F0%9F%93%A6%EF%B8%8Fmodule.json",
      "status": 200,
      "token": "module-route-proof"
    }
  ],
  "imported": "dynamic-import-proof"
}
```

Observed CDN responses:

```json
[
  {
    "url": "https://map.assets.semio-tech.com/osm/%F0%9F%A7%AA%EF%B8%8Frouting/%F0%9F%8C%8D%EF%B8%8Fmap.json",
    "status": 200,
    "cors": "*",
    "mime": "application/json;charset=utf-8"
  },
  {
    "url": "https://media.assets.semio-tech.com/%F0%9F%96%BC%EF%B8%8Fassets/%F0%9F%A7%AA%EF%B8%8Frouting/%F0%9F%96%BC%EF%B8%8Fmedia.json",
    "status": 200,
    "cors": "*",
    "mime": "application/json;charset=utf-8"
  },
  {
    "url": "https://modules.assets.semio-tech.com/%F0%9F%94%8C%EF%B8%8Fplugin-modules/%F0%9F%A7%AA%EF%B8%8Frouting/%F0%9F%93%A6%EF%B8%8Fmodule.json",
    "status": 200,
    "cors": "*",
    "mime": "application/json;charset=utf-8"
  },
  {
    "url": "https://modules.assets.semio-tech.com/%F0%9F%94%8C%EF%B8%8Fplugin-modules/%F0%9F%A7%AA%EF%B8%8Frouting/%F0%9F%A7%A9%EF%B8%8Fimport.js",
    "status": 200,
    "cors": "*",
    "mime": "text/javascript;charset=utf-8"
  }
]
```

Real browser console proof:

```text
[DEBUG] Worker CDN fetch https://map.assets.semio-tech.com/osm/%F0%9F%A7%AA%EF%B8%8Frouting/%F0%9F%8C%8D%EF%B8%8Fmap.json 200 map-route-proof
[DEBUG] Worker CDN fetch https://media.assets.semio-tech.com/%F0%9F%96%BC%EF%B8%8Fassets/%F0%9F%A7%AA%EF%B8%8Frouting/%F0%9F%96%BC%EF%B8%8Fmedia.json 200 media-route-proof
[DEBUG] Worker CDN fetch https://modules.assets.semio-tech.com/%F0%9F%94%8C%EF%B8%8Fplugin-modules/%F0%9F%A7%AA%EF%B8%8Frouting/%F0%9F%93%A6%EF%B8%8Fmodule.json 200 module-route-proof
[DEBUG] CDN dynamic module evaluated
[DEBUG] Worker CDN dynamic import https://modules.assets.semio-tech.com/🔌️plugin-modules/🧪️routing/🧩️import.js dynamic-import-proof
```

This validates the route adapter and worker browser behavior, not the fresh build's generated components or actual hosting-provider configuration.
