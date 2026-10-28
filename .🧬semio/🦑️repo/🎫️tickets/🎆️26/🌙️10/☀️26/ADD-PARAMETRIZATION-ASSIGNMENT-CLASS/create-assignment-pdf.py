"""📝 Create the one-off printable German assignment from the reviewed source contract."""

import json
from io import BytesIO
import math
from pathlib import Path

from pypdf import PdfReader, PdfWriter
from pypdf.generic import NameObject, TextStringObject

import reportlab
from reportlab.lib import colors
from reportlab.lib.enums import TA_CENTER
from reportlab.lib.pagesizes import A4
from reportlab.lib.styles import ParagraphStyle
from reportlab.lib.units import mm
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.pdfgen import canvas
from reportlab.platypus import Paragraph

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework/🛍️products/📓️print").exists())
PRINT = ROOT / "🧰️framework/🛍️products/📓️print"
FIXTURE = json.loads((PRINT / "🧪️tests/📝️parametrization-assignment/🧫️cases.json").read_text(encoding="utf-8"))
TEX = (PRINT / "🧾️template/📝️assignment/parametrization-de.content.tex").read_text(encoding="utf-8")
GEOMETRY = (PRINT / "🧾️template/📝️assignment/parametrization-geometry.tex").read_text(encoding="utf-8")
OUTPUT = TICKET / "🗑️generated/Fingeruebung-Parametrisierung.pdf"
OUTPUT.parent.mkdir(parents=True, exist_ok=True)
assert "Fingerübung Parametrisierung" in TEX
assert TEX.count(r"\clearpage") == 4
assert len(FIXTURE["exercises"]) == 2
EDGES = [(a, b) for a in range(1, 9) for b in range(a + 1, 9)]
assert len(EDGES) == 28
for exercise in FIXTURE["exercises"]:
    assert len(exercise["types"]) == 8
    for kind, length, diameter in exercise["types"]:
        assert f"#1{{{kind}}}{{{length}}}{{{diameter}}}" in GEOMETRY
ANTAFONT = PRINT / "🖼️assets/🔤️font/🅰️anta/🅰️Anta-Regular.ttf"
pdfmetrics.registerFont(TTFont("Anta", str(ANTAFONT)))
BODY_FONTS = Path(reportlab.__file__).parent / "fonts"
pdfmetrics.registerFont(TTFont("Body", str(BODY_FONTS / "Vera.ttf")))
pdfmetrics.registerFont(TTFont("Body-Bold", str(BODY_FONTS / "VeraBd.ttf")))
pdfmetrics.registerFontFamily("Body", normal="Body", bold="Body-Bold", italic="Body", boldItalic="Body-Bold")
C = canvas.Canvas(str(OUTPUT), pagesize=A4, pageCompression=1)
C.setTitle("Fingerübung Parametrisierung")
C.setAuthor("Semio - Architektur")
C.setSubject("Parametrisches Entwerfen: Eindeutigkeit, Konsistenz und Minimalität")
INK = colors.HexColor("#171717")
MUTED = colors.HexColor("#555555")
RULE = colors.HexColor("#888888")
PALE = colors.HexColor("#F2F2F2")
STYLES = {
    "body": ParagraphStyle("body", fontName="Body", fontSize=10.5, leading=14.2, textColor=INK, spaceAfter=0),
    "small": ParagraphStyle("small", fontName="Body", fontSize=9.4, leading=12.6, textColor=INK),
    "center": ParagraphStyle("center", fontName="Body", fontSize=10.5, leading=14.2, alignment=TA_CENTER, textColor=INK),
}


def line(x1, t1, x2, t2, width=.45, color=RULE):
    """📏 Draw a rule in page coordinates measured down from the top."""
    C.setStrokeColor(color)
    C.setLineWidth(width)
    C.line(x1 * mm, (297 - t1) * mm, x2 * mm, (297 - t2) * mm)


def text(value, x, top, size=10.5, font="Body", color=INK):
    """🔤 Place a text baseline in millimetres from the upper page edge."""
    C.setFillColor(color)
    C.setFont(font, size)
    C.drawString(x * mm, (297 - top) * mm, value)


def paragraph(value, top, x=20, width=170, style="body"):
    """📄 Lay out a measured paragraph and return its lower edge."""
    item = Paragraph(value, STYLES[style])
    _, height = item.wrap(width * mm, 800)
    item.drawOn(C, x * mm, (297 - top) * mm - height)
    return top + height / mm


def heading(value, top, number=None, size=13):
    """🏷 Draw the common section title."""
    text(value, 20, top, size, "Anta")
    if number:
        C.setFont("Body", 8)
        C.setFillColor(MUTED)
        C.drawRightString(190 * mm, (297 - top) * mm, number)


def begin_page(page, label):
    """📑 Establish the running header and footer for one A4 page."""
    text("SEMIO  /  ARCHITEKTUR", 20, 15, 7.2, "Body-Bold")
    C.setFont("Body", 7.2)
    C.setFillColor(MUTED)
    C.drawRightString(190 * mm, 282 * mm, "PARAMETRISCHES ENTWERFEN")
    line(20, 19, 190, 19, .6, INK)
    line(20, 278, 190, 278, .4, RULE)
    text(label, 20, 284, 7.4, color=MUTED)
    C.setFont("Body", 7.4)
    C.drawRightString(190 * mm, 13 * mm, f"{page} / 5")


def shape(kind, length, diameter, cx, top, scale=1):
    """💊 Draw the authored capsule silhouette without answers or labels."""
    C.saveState()
    C.translate((cx - length * scale / 2) * mm, (297 - top - diameter * scale / 2) * mm)
    C.scale(mm * scale, mm * scale)
    C.setStrokeColor(INK)
    C.setLineWidth(.22 / scale)
    radius = diameter / 2
    k = .55228475
    if kind == "capsule":
        path = C.beginPath()
        path.moveTo(radius, 0)
        path.lineTo(length - radius, 0)
        path.curveTo(length - radius + k * radius, 0, length, radius - k * radius, length, radius)
        path.curveTo(length, radius + k * radius, length - radius + k * radius, diameter, length - radius, diameter)
        path.lineTo(radius, diameter)
        path.curveTo(radius - k * radius, diameter, 0, radius + k * radius, 0, radius)
        path.curveTo(0, radius - k * radius, radius - k * radius, 0, radius, 0)
        C.drawPath(path)
        C.setLineWidth(.13 / scale)
        C.setStrokeColor(MUTED)
        for join in [radius, length - radius]:
            C.arc(join - diameter / 7, 0, join + diameter / 7, diameter, -90, 180)
    else:
        C.line(1, 0, length - 1, 0)
        C.line(1, diameter, length - 1, diameter)
        C.ellipse(0, 0, 2, diameter)
        C.arc(length - 2, 0, length, diameter, -90, 180)
        C.setDash(.8, .6)
        C.setStrokeColor(MUTED)
        C.setLineWidth(.13 / scale)
        C.arc(length - 2, 0, length, diameter, 90, 180)
    C.restoreState()


def box(length, width, height, cx, top, label):
    """📦 Render a common-scale axonometric box and its type label."""
    unit = 6.5
    depth_x = width * .45 * unit
    depth_y = width * .35 * unit
    front_width, front_height = length * unit, height * unit
    left = cx - (front_width + depth_x) / 2
    bottom = top + 17
    points = [(left, bottom), (left + front_width, bottom), (left + front_width, bottom - front_height), (left, bottom - front_height)]
    C.setStrokeColor(INK)
    C.setLineWidth(.65)
    path = C.beginPath()
    path.moveTo(points[0][0] * mm, (297 - points[0][1]) * mm)
    for x, y in points[1:]:
        path.lineTo(x * mm, (297 - y) * mm)
    path.close()
    C.drawPath(path)
    a, b, c = points[3], points[2], points[1]
    for p, q in [(a, (a[0] + depth_x, a[1] - depth_y)), ((a[0] + depth_x, a[1] - depth_y), (b[0] + depth_x, b[1] - depth_y)), ((b[0] + depth_x, b[1] - depth_y), b), (c, (c[0] + depth_x, c[1] - depth_y)), ((c[0] + depth_x, c[1] - depth_y), (b[0] + depth_x, b[1] - depth_y))]:
        line(*p, *q, .65, INK)
    paragraph(label, bottom + 3, cx - 20, 40, "center")


def worked_example():
    """🧮 Print the complete three-box example and isolated counterexamples."""
    begin_page(1, "Einführung und Beispiel")
    text("Fingerübung Parametrisierung", 20, 33, 23, "Anta")
    text("Von Formen zu einem stimmigen Entwurfsraum", 20, 41, 10, color=MUTED)
    text("45-60 Minuten", 20, 51, 9, "Body-Bold")
    text("Abgabe: zwei Tabellen und zwei Übergangsgraphen", 64, 51, 9)
    heading("Ziel", 64)
    paragraph("Entwickeln Sie ein Gefühl für mögliche Parametrisierungen. Finden Sie Parameter, die eine Familie von Formen beschreiben. Ein Typ ist eine eigenständige Geometrie; Orientierung und Einheiten bleiben gleich.", 68)
    heading("Drei Quader: Länge L, Breite B, Höhe H", 91)
    for dims, cx, name in zip(FIXTURE["boxes"], [48, 105, 162], ["A", "B", "C"]):
        box(*dims, cx, 95, name)
    top = 127
    widths = [50, 40, 40, 40]
    xs = [20, 70, 110, 150, 190]
    C.setFillColor(PALE)
    C.rect(20 * mm, (297 - top - 6) * mm, 170 * mm, 6 * mm, fill=1, stroke=0)
    for offset, label in [(0, "Typ"), (1, "Länge L"), (2, "Breite B"), (3, "Höhe H")]:
        text(label, xs[offset] + 3, top + 4.2, 9, "Body-Bold")
    for row, (name, values) in enumerate(zip(["A", "B", "C"], FIXTURE["boxes"])):
        y = top + 6 + row * 5.2
        for col, value in enumerate([name, *values]):
            text(str(value), xs[col] + 3, y + 3.8, 9)
        line(20, y + 5.2, 190, y + 5.2, .3)
    paragraph("<b>Gute Beschreibung:</b> Ein Tupel (L, B, H) erzeugt genau einen Quader. A nach B: nur L +1. B nach C: nur B +1. A nach C: L +1 und B +1. Der Zyklus A - B - C - A summiert sich zu (0, 0, 0). Alle Maße gelten in derselben freien Einheit und entlang derselben Achsen.", 152, style="small")
    heading("Drei Kriterien - drei getrennte Fehler", 179)
    blocks = [
        ("Eindeutigkeit", "Jede zulässige Kombination bestimmt genau einen Typ.", "<b>Schlecht:</b> Nur (L, H) verwenden. B und C erhalten beide (3, 1), obwohl ihre Breiten verschieden sind. Die fehlende Breite macht das Ergebnis mehrdeutig."),
        ("Konsistenz", "Gerichtete Änderungen sind wegunabhängig; geschlossene Zyklen führen zu den Ausgangswerten zurück.", "<b>Schlecht:</b> Bei der eindeutigen Regel (L, B, H) fälschlich A nach C = (+1, +2, 0) vorschreiben. Über B gilt (+1, +1, 0); die Zyklussumme ist (0, -1, 0). Nur die Übergangsregel ist falsch."),
        ("Minimalität", "Ändern Sie so wenige Parameterwerte, wie die Geometrie erfordert.", "<b>Schlecht:</b> (p, q, r) = (L, L+B, H): A = (2, 3, 1), B = (3, 4, 1), C = (3, 5, 1). Umkehrbar durch (L, B, H) = (p, q-p, r), also eindeutig und konsistent. Für A nach B ändern sich p und q; bei (L, B, H) genügt eine Änderung."),
    ]
    cursor = 184
    for name, rule, bad in blocks:
        line(20, cursor + .5, 20, cursor + 5.5, 1.5, INK)
        cursor = paragraph(f"<b>{name}.</b> {rule} {bad}", cursor, x=24, width=166, style="small") + 4
    assert cursor < 276, f"first page overflow {cursor}"
    C.showPage()


def exercise_table(index):
    """🗒 Print one blank parameter table with eight alternating unnamed drawings."""
    page = 2 if index == 0 else 4
    begin_page(page, f"Übung {index + 1} / Parametertabelle")
    heading(f"Übung {index + 1}: Typen beschreiben", 33, "FORMEN / PARAMETER", 19)
    intro = "Entwickeln Sie eine Parametrisierung für die acht Formen." if index == 0 else "Parametrisieren Sie diese zweite Familie unabhängig. Erproben Sie eine andere Parameterstrategie."
    intro_bottom = paragraph(intro + " Schreiben Sie Parameternamen in die fünf leeren Kopfzellen und Werte in die Zeilen. Nutzen Sie nur so viele Spalten wie nötig; ungenutzte Spalten bleiben leer.", 41)
    explanation_bottom = paragraph("Die Formen wechseln zwischen runden Kapseln und zylindrischen Kapseln mit ebenen Stirnflächen. Die Endform gehört zum Typ. Behalten Sie die gezeigte Achsrichtung bei.", intro_bottom + 3, style="small")
    assert explanation_bottom <= 75, f"exercise intro overlap {explanation_bottom}"
    top, row_h, header_h, first = 78, 18, 16, 36
    total_h = header_h + row_h * 8
    C.setFillColor(PALE)
    C.rect(20 * mm, (297 - top - header_h) * mm, first * mm, header_h * mm, fill=1, stroke=0)
    text("Parameter", 23, top + 9.5, 10, "Body-Bold")
    for i in range(9):
        y = top + header_h + i * row_h
        line(20, y, 190, y, .4 if i < 8 else .65, INK)
    line(20, top, 190, top, .65, INK)
    for x in [20, 20 + first] + [20 + first + (170 - first) / 5 * i for i in range(1, 6)]:
        line(x, top, x, top + total_h, .4, INK)
    for row, values in enumerate(FIXTURE["exercises"][index]["types"]):
        shape(*values, 20 + first / 2, top + header_h + (row + .5) * row_h, 1.15)
    heading("Beschreibung prüfen" if index == 0 else "Entscheidungen begründen", 249, size=11)
    question = "Bestimmt jede Kombination genau einen Typ? Kann eine andere Person ihn ohne die Zeichnung rekonstruieren?" if index == 0 else "Welche geometrischen Entscheidungen steuern Ihre Parameter? Welche Wahl erleichtert Übergänge?"
    paragraph(question, 253, style="small")
    line(20, 269, 190, 269, .3)
    C.showPage()


def graph(index, top=104):
    """🕸 Repeat the exact row geometries at the vertices of a complete graph."""
    cx, radius = 105, 33
    positions = []
    for vertex in range(8):
        angle = math.radians(90 - 45 * vertex)
        positions.append((cx + radius * math.cos(angle), top - radius * math.sin(angle)))
    for a, b in EDGES:
        line(*positions[a - 1], *positions[b - 1], .4, RULE)
    for row, (values, (x, y)) in enumerate(zip(FIXTURE["exercises"][index]["types"], positions)):
        kind, length, diameter = values
        C.setFillColor(colors.white)
        C.rect((x - length * .32 - 1.5) * mm, (297 - y - diameter * .32 - 1.5) * mm,
               (length * .64 + 3) * mm, (diameter * .64 + 3) * mm, fill=1, stroke=0)
        shape(*values, x, y, .64)
        angle = math.radians(90 - 45 * row)
        label_x = x + 9 * math.cos(angle)
        label_y = y - 9 * math.sin(angle)
        C.setFillColor(INK)
        C.setFont("Body-Bold", 9)
        C.drawCentredString(label_x * mm, (297 - label_y) * mm - 3, str(row + 1))


def transitions(index):
    """🔗 Provide one distinct writing line for each of the twenty-eight edges."""
    page = 3 if index == 0 else 5
    begin_page(page, f"Übung {index + 1} / Übergangsgraph und Änderungen")
    heading(f"Übung {index + 1}: Jedes Paar verbinden", 33, size=18)
    paragraph("Die Knoten 1 bis 8 entsprechen den Tabellenzeilen von oben nach unten. Jede Form ist mit jeder anderen verbunden: Der vollständige Graph K8 hat 8 × 7 / 2 = 28 ungerichtete Kanten.", 41)
    graph(index)
    paragraph("Notieren Sie für jede Kante die geänderten Parameter und deren gerichtete Änderungen. Lesen Sie von der kleineren zur größeren Zeilennummer. Für den Rückweg kehren sich die Vorzeichen um.", 151, style="small")
    text("Kante", 20, 171, 9, "Body-Bold", MUTED)
    text("Parameteränderungen", 35, 171, 8.4, "Body-Bold", MUTED)
    text("Kante", 108, 171, 9, "Body-Bold", MUTED)
    text("Parameteränderungen", 123, 171, 8.4, "Body-Bold", MUTED)
    for entry, (a, b) in enumerate(EDGES):
        column, row = divmod(entry, 14)
        x, y = 20 + column * 88, 178 + row * 5.8
        text(f"{a} - {b}", x, y, 9)
        line(x + 15, y + 1, min(x + 82, 190), y + 1, .3)
    check = "<b>Zyklustest:</b> Wählen Sie ein Dreieck und addieren Sie seine gerichteten Änderungen. Kehren Sie zu null zurück? Kann ein Übergang weniger Parameterwerte ändern?" if index == 0 else "<b>Vergleich:</b> Welche Beschreibung ist eindeutig, konsistent und sparsam? Begründen Sie Ihr Urteil mit einem Übergang und einem geschlossenen Zyklus."
    paragraph(check, 262, style="small")
    C.showPage()


worked_example()
exercise_table(0)
transitions(0)
exercise_table(1)
transitions(1)
C.save()
reader = PdfReader(BytesIO(OUTPUT.read_bytes()))
writer = PdfWriter()
writer.clone_document_from_reader(reader)
writer.root_object[NameObject("/Lang")] = TextStringObject("de-DE")
with OUTPUT.open("wb") as target:
    writer.write(target)
assert len(PdfReader(OUTPUT).pages) == 5
print("Created five-page German assignment PDF")
