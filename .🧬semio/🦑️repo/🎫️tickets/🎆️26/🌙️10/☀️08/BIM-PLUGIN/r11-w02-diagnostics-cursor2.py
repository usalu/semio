import sys
E = sys.argv[1]
M = sys.argv[2]
keys = [("Left","left"),("Right","right"),("Up","up"),("Down","down"),("LeftFar","left_far"),("RightFar","right_far"),("UpFar","up_far"),("DownFar","down_far"),("Place","place")]
def camel(name):
    return name[0].lower() + name[1:]
def kebab(label):
    out = ''
    for ch in label:
        out += ('-' + ch.lower()) if ch.isupper() else ch
    return out.lstrip('-')
def patch(path, pairs):
    s = open(path, encoding='utf-8', newline='').read()
    for old, new in pairs:
        assert s.count(old) == 1, (path, old, s.count(old))
        s = s.replace(old, new)
    open(path + ".new", 'w', encoding='utf-8', newline='').write(s)
rows = ''
arms = ''
for name, snake in keys:
    cid = "cursor" + name
    rows += '            "%s" as "%s" => cursor_keys::Cursor%s, [Artifact, WindowTransient]; Mutation, cmd_cursor_%s, cmd_cursor_%s_describe;\n' % (cid, kebab(cid), name, snake, snake)
    arms += '            "%s" => BimCommand::Cursor%s(decode(action, fold(args, &[], &[]))?),\n' % (cid, name)
ids = ', '.join('"cursor%s"' % name for name, _ in keys)
bind = ', '.join(('("mod+%s", "cursor%s")' % (("arrow" + n.lower()), n)) for n in ["Left","Right","Up","Down"])
bind_far = ', '.join(('("mod+shift+arrow%s", "cursor%sFar")' % (n.lower(), n)) for n in ["Left","Right","Up","Down"])
patch(E + "/🦀️.rs", [
    ("remove_classification, remove_property,", "cursor_keys, remove_classification, remove_property,"),
    ('            "engagementInput" as "engagement-input" =>', rows + '            "engagementInput" as "engagement-input" =>'),
    ('            "engagementInput" => BimCommand::EngagementInput(', arms + '            "engagementInput" => BimCommand::EngagementInput('),
])
patch(E + "/🧵️gestures/🦀️.rs", [
    ('"engagementInput", "engagementSubmit"];', '"engagementInput", "engagementSubmit", ' + ids + '];'),
    ('&[("escape", "canvasEscape"), ("enter", "canvasCommitDraft")];', '&[("escape", "canvasEscape"), ("enter", "canvasCommitDraft"), ' + bind + ', ' + bind_far + ', ("mod+enter", "cursorPlace")];'),
    ("/// ⌨️ The keys that drive a gesture in progress: Escape cancels it, Enter finishes it.", "/// ⌨️ The keys that drive a gesture in progress: Escape cancels it, Enter finishes it, the arrow keys with the command key move the keyboard cursor by a fine step and with shift by a coarse one, and Enter with the command key clicks at the cursor."),
])
