import pathlib

path = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs")
text = path.read_text()


def replace(old, new):
    global text
    assert text.count(old) == 1, (text.count(old), old[:90])
    text = text.replace(old, new)


replace("            let mut replay_capture_started = false;\n", "            let mut replay_capture_started = false;\n            let mut reserved_step_published = false;\n")
replace("                                job.step = next;\n", "                                job.step = next;\n                                reserved_step_published = true;\n")
replace("return if replay_capture_started || stepped_reserved_job.is_some() {", "return if replay_capture_started || reserved_step_published {")
path.write_text(text)
print("ok")
