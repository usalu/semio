import sys, os
sys.path.insert(0, "/private/tmp/claude-501/-Users-ueli-Documents-semio/43bfe996-fced-47cc-b279-32d897c6af08/scratchpad")
from split_feature import parse_and_split

ARTIFACTS = [
  {
    "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🧪️tests/mutate-las-1-0/🥒️.feature",
    "base_catalog_id": "las-1-0-any", "base_capability": "las-1-0-any-mutate",
    "satellites": [
      {"name":"vlr","dirname":"mutate-las-1-0-vlr","catalog_id":"las-1.0-vlr","capability":"las-1-0-vlr-mutate",
       "kinds":["insert-vlr","remove-vlr","set-vlr-data"]},
      {"name":"points","dirname":"mutate-las-1-0-points","catalog_id":"las-1.0-points","capability":"las-1-0-points-mutate",
       "kinds":["insert-point","remove-point","set-point"]},
    ],
  },
  {
    "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🧪️tests/mutate-gif-89a/🥒️.feature",
    "base_catalog_id": "gif-89a-any", "base_capability": "gif-89a-mutate",
    "satellites": [
      {"name":"graphic-control","dirname":"mutate-gif-89a-graphic-control","catalog_id":"gif-89a-graphic-control","capability":"gif-89a-graphic-control-mutate",
       "kinds":["set-frame-delay","set-frame-disposal","set-frame-transparency","set-frame-user-input"]},
      {"name":"comment","dirname":"mutate-gif-89a-comment","catalog_id":"gif-89a-comment","capability":"gif-89a-comment-mutate",
       "kinds":["insert-comment","remove-comment"]},
      {"name":"application","dirname":"mutate-gif-89a-application","catalog_id":"gif-89a-application","capability":"gif-89a-application-mutate",
       "kinds":["add-app-extension","remove-app-extension","set-loop-count"]},
    ],
  },
  {
    "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj/🧪️tests/mutate-obj-3-0/🥒️.feature",
    "base_catalog_id": "obj-3-0-any", "base_capability": "obj-3-0-mutate",
    "satellites": [
      {"name":"material","dirname":"mutate-obj-3-0-material","catalog_id":"obj-3.0-material","capability":"obj-3-0-material-mutate",
       "kinds":["set-mtllib","set-usemtl"]},
    ],
  },
  {
    "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dxf/🧪️tests/mutate-dxf-r12/🥒️.feature",
    "base_catalog_id": "dxf-r12-any", "base_capability": "dxf-r12-mutate",
    "satellites": [
      {"name":"tables","dirname":"mutate-dxf-r12-tables","catalog_id":"dxf-r12-tables","capability":"dxf-r12-tables-mutate",
       "kinds":["insert-layer","remove-layer","set-layer","insert-style","remove-style","set-style","insert-linetype","remove-linetype","set-linetype"]},
      {"name":"blocks","dirname":"mutate-dxf-r12-blocks","catalog_id":"dxf-r12-blocks","capability":"dxf-r12-blocks-mutate",
       "kinds":["insert-block","remove-block","set-block"]},
      {"name":"entities","dirname":"mutate-dxf-r12-entities","catalog_id":"dxf-r12-entities","capability":"dxf-r12-entities-mutate",
       "kinds":["insert-entity","remove-entity","set-entity"]},
    ],
  },
  {
    "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🧪️tests/mutate-bcf-2-1/🥒️.feature",
    "base_catalog_id": "bcf-2-1-any", "base_capability": "bcf-2-1-mutate",
    "satellites": [
      {"name":"viewpoint","dirname":"mutate-bcf-2-1-viewpoint","catalog_id":"bcf-2.1-viewpoint","capability":"bcf-2-1-viewpoint-mutate",
       "kinds":["insert-viewpoint","remove-viewpoint","set-viewpoint-camera","set-viewpoint-components"]},
      {"name":"snapshot","dirname":"mutate-bcf-2-1-snapshot","catalog_id":"bcf-2.1-snapshot","capability":"bcf-2-1-snapshot-mutate",
       "kinds":["set-viewpoint-snapshot"]},
    ],
  },
  {
    "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🧪️tests/mutate-avi-1-0/🥒️.feature",
    "base_catalog_id": "avi-1-0-any", "base_capability": "avi-1-0-mutate",
    "satellites": [
      {"name":"movi","dirname":"mutate-avi-1-0-movi","catalog_id":"avi-1.0-movi","capability":"avi-1-0-movi-mutate",
       "kinds":["insert-chunk","remove-chunk","set-chunk-keyframe","add-unknown-chunk","remove-unknown-chunk"]},
      {"name":"idx1","dirname":"mutate-avi-1-0-idx1","catalog_id":"avi-1.0-idx1","capability":"avi-1-0-idx1-mutate",
       "kinds":["set-idx1-present"]},
    ],
  },
]

for cfg in ARTIFACTS:
    path = cfg["path"]
    results, base_kinds, all_kinds = parse_and_split(path, cfg["base_catalog_id"], cfg["base_capability"], cfg["satellites"])
    # write base (overwrite in place)
    with open(path, "w", encoding="utf-8") as f:
        f.write(results["__base__"])
    print(path, "BASE kinds:", len(base_kinds))
    art_root = os.path.dirname(os.path.dirname(path))  # .../🧪️tests -> artifact root
    tests_dir = os.path.dirname(path)  # .../🧪️tests/mutate-x  -> wait path already includes mutate-x/🥒️.feature
    tests_root = os.path.dirname(os.path.dirname(path))  # artifact/🧪️tests
    for sat in cfg["satellites"]:
        outdir = f"{os.path.dirname(os.path.dirname(path))}/{sat['dirname']}"
        os.makedirs(outdir, exist_ok=True)
        outpath = f"{outdir}/🥒️.feature"
        with open(outpath, "w", encoding="utf-8") as f:
            f.write(results[sat["name"]])
        print("  wrote", outpath, "kinds:", len(sat["kinds"]))
