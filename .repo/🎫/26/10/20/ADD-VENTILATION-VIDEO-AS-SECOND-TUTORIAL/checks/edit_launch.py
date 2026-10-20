import re, pathlib
p = pathlib.Path(".vscode/launch.json")
s = p.read_text()

def bump(m):
    n = int(m.group(1))
    return f'"order": {n + 3}' if 210 <= n <= 215 else m.group(0)

head, sep, tail = s.partition('"name": "🛠️dev🎬manim physical fundamentals full"')
assert sep
pf_full_end = tail.index("    },\n") + len("    },\n")
rest = re.sub(r'"order": (\d+)', bump, tail[pf_full_end:])

dev = '''    {
      "name": "🛠️dev🎬manim ventilation",
      "type": "node-terminal",
      "request": "launch",
      "command": "uv run manim -pql tutorial/energy/demand/Ventilation/1_warum_lueften/scene_1.py Beat1_Lueftungsbedarf",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 210
      }
    },
    {
      "name": "🛠️dev🎬manim ventilation full",
      "type": "node-terminal",
      "request": "launch",
      "command": "\\"${workspaceFolder}/.venv/bin/python\\" tutorial/energy/demand/Ventilation/full_ventilation_video.py -q h --no-play --force",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 211
      }
    },
    {
      "name": "🛠️dev🎬manim intro ventilation",
      "type": "node-terminal",
      "request": "launch",
      "command": "uv run manim -pql tutorial/intro/intro_scene.py Demo_Intro_Lueftung",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 212
      }
    },
'''
s = head + sep + tail[:pf_full_end] + dev + rest

chapters = [
    ("lueftungsbedarf", "1_warum_lueften"),
    ("luftmenge", "2_luftmenge"),
    ("waermestromluft", "3_waermestrom_luft"),
    ("natuerlichelueftung", "4_natuerliche_lueftung"),
    ("mechanischelueftung", "5_mechanische_lueftung"),
    ("uebergabe", "6_uebergabe"),
]
kinds = [("🔊audio", "generate_audio.py"), ("📍trace", "generate_audio.py --trace"),
         ("🧩align", "generate_audio.py --align"), ("🔎syncreport", "generate_audio.py --report"),
         ("🎬video", "build_full_video.py")]
order = 144
build = ""
for slug, folder in chapters:
    for icon, cmd in kinds:
        script, _, flag = cmd.partition(" ")
        full = f"tutorial/energy/demand/Ventilation/{folder}/{script}" + (f" {flag}" if flag else "")
        build += f'''    {{
      "name": "📦build📽️{slug}{icon}",
      "type": "node-terminal",
      "request": "launch",
      "command": "uv run python {full}",
      "cwd": "${{workspaceFolder}}",
      "presentation": {{
        "group": "4_build",
        "order": {order}
      }}
    }},
'''
        order += 1
marker = "    // #endregion\n    // #region ⬆️ Publish"
assert s.count(marker) == 1
s = s.replace(marker, build + marker)
p.write_text(s)
