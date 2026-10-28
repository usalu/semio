import pathlib

p = next(pathlib.Path("C:/git/semio/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧩️models").glob("*.py"))
s = p.read_text(encoding="utf-8")


def sub(old, new):
    global s
    assert s.count(old) == 1, old
    s = s.replace(old, new)


sub('"""🎬 Play the stages with the clause', '"""📽️ Play the stages with the clause')
sub('"""🚪 Beat 3:', '"""🛋️ Beat 3:')
sub('"""🏢 The architecture layer in material colours', '"""🏬 The architecture layer in material colours')
sub('"""🏷️ Beat 17:', '"""📇 Beat 17:')
sub('"""🧮 A formula line centred', '"""➗ A formula line centred')
sub('"""🧮 The analysis model without its node dots', '"""🕸️ The analysis model without its node dots')
sub('"""🧱 Beat 7: explode', '"""🧲 Beat 7: explode')
p.write_text(s, encoding="utf-8")
print("ok")
