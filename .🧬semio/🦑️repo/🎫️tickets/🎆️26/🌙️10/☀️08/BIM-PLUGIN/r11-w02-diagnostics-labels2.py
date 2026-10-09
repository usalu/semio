import sys
path = sys.argv[1]
s = open(path, encoding='utf-8', newline='').read()
anchor = "    sch_surface_describe:"
assert s.count(anchor) == 1 and "cmd_cursor_left:" not in s
rows = [
    ("left", "Cursor Left", "Cursor nach links", "Moves the keyboard cursor of the armed tool 0.1 m to the left.", "Bewegt den Tastaturcursor des aktiven Werkzeugs um 0,1 m nach links."),
    ("right", "Cursor Right", "Cursor nach rechts", "Moves the keyboard cursor of the armed tool 0.1 m to the right.", "Bewegt den Tastaturcursor des aktiven Werkzeugs um 0,1 m nach rechts."),
    ("up", "Cursor Up", "Cursor nach oben", "Moves the keyboard cursor of the armed tool 0.1 m up.", "Bewegt den Tastaturcursor des aktiven Werkzeugs um 0,1 m nach oben."),
    ("down", "Cursor Down", "Cursor nach unten", "Moves the keyboard cursor of the armed tool 0.1 m down.", "Bewegt den Tastaturcursor des aktiven Werkzeugs um 0,1 m nach unten."),
    ("left_far", "Cursor Far Left", "Cursor weit nach links", "Moves the keyboard cursor of the armed tool 1 m to the left.", "Bewegt den Tastaturcursor des aktiven Werkzeugs um 1 m nach links."),
    ("right_far", "Cursor Far Right", "Cursor weit nach rechts", "Moves the keyboard cursor of the armed tool 1 m to the right.", "Bewegt den Tastaturcursor des aktiven Werkzeugs um 1 m nach rechts."),
    ("up_far", "Cursor Far Up", "Cursor weit nach oben", "Moves the keyboard cursor of the armed tool 1 m up.", "Bewegt den Tastaturcursor des aktiven Werkzeugs um 1 m nach oben."),
    ("down_far", "Cursor Far Down", "Cursor weit nach unten", "Moves the keyboard cursor of the armed tool 1 m down.", "Bewegt den Tastaturcursor des aktiven Werkzeugs um 1 m nach unten."),
    ("place", "Place at Cursor", "Am Cursor platzieren", "Clicks the armed tool at the keyboard cursor, as a click at that exact point would.", "Klickt mit dem aktiven Werkzeug am Tastaturcursor, wie ein Klick auf genau diesen Punkt."),
]
block = ''
for key, en, de, den, dde in rows:
    block += '    cmd_cursor_%s: "%s", "%s";\n    cmd_cursor_%s_describe: "%s", "%s";\n' % (key, en, de, key, den, dde)
s = s.replace(anchor, block + anchor)
open(path + ".new", 'w', encoding='utf-8', newline='').write(s)
