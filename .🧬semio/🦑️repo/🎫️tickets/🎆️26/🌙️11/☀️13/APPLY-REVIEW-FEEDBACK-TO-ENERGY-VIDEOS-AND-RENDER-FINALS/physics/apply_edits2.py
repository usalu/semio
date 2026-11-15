from pathlib import Path

ROOT = Path(r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\🧲️physics\🎬️clip\🧲️fundamentals\🐍️.py")
text = ROOT.read_text(encoding="utf-8")
failures = []


def sub(old, new, count=1):
    global text
    found = text.count(old)
    if found != count:
        failures.append((found, count, old[:80]))
        return
    text = text.replace(old, new)


sub(r'times irradiance $I$."', r'times irradiance $I_{\mathrm{s}}$."')
sub(r'mal Einstrahlung $I$."', r'mal Einstrahlung $I_{\mathrm{s}}$."')
sub(r'i_lbl = _m(r"I = 500', r'i_lbl = _m(r"I_{\mathrm{s}} = 500')
sub(r'gi_lbl = _m(r"g_{\perp} \cdot I \approx', r'gi_lbl = _m(r"g_{\perp} \cdot I_{\mathrm{s}} \approx')
sub(r'("i", "I", P_YELLOW)', r'("i", r"I_{\mathrm{s}}", P_YELLOW)')
sub(r'r"\dot{Q}_{\mathrm{S}} = g_{\perp} \cdot A \cdot I"),', r'r"\dot{Q}_{\mathrm{S}} = g_{\perp} \cdot A \cdot I_{\mathrm{s}}"),')
sub(r'tr(r"\text{DIN EN 410: } g_{\perp} \text{ of glazing}", r"\text{DIN EN 410: } g_{\perp} \text{ der Verglasung}")',
    r'tr(r"\text{DIN EN 410 · } g_{\perp}", r"\text{DIN EN 410 · } g_{\perp}")')
sub('np.array([-6.9, -1.9, 0.0]), size=LABEL_FONT_SIZE, color=P_PRIMARY, edge="left")', 'np.array([-6.9, -1.15, 0.0]), size=LABEL_FONT_SIZE, color=P_PRIMARY, edge="left")')
sub(r'tr(r"\text{DIN EN ISO 13786: dynamic thermal behaviour (heat storage)}", r"\text{DIN EN ISO 13786: dynamisch-thermisches Verhalten (Wärmespeicherung)}")',
    r'tr(r"\text{DIN EN ISO 13786 · heat storage}", r"\text{DIN EN ISO 13786 · Wärmespeicherung}")')
sub('cpt(18, 33.9),', 'cpt(18, 35.2),')
sub('''            lines.append(Text(f"{edition} · {topics}" if topics else edition, font_size=LABEL_FONT_SIZE - 3, color=P_WHITE).set_opacity(0.7))''',
    '''            lines.append(Text(edition, font_size=LABEL_FONT_SIZE - 3, color=P_WHITE).set_opacity(0.7))
            if topics:
                lines.append(Text(topics, font_size=LABEL_FONT_SIZE - 3, color=P_WHITE).set_opacity(0.7))''')
ROOT.write_text(text, encoding="utf-8")
print(failures)
