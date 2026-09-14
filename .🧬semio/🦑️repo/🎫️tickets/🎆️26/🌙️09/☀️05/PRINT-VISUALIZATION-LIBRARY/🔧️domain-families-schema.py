# -*- coding: utf-8 -*-
"""🔧️ Writes the option vocabulary of the sixteen FAMILIES-DOMAIN families into the schema.

The vocabularies are the `%region 🔖️Keys-<family>` blocks of `semio-viz-text.sty` and
`semio-viz-domain.sty`, transcribed with their type, their default and a bilingual description.
The schema is re-read immediately before the write and written atomically because other agents
edit the vocabularies of their own families at the same time.
"""

import io
import json
import os
import sys

SCHEMA = sys.argv[1]
OWNER = "FAMILIES-DOMAIN"


def option(kind, en, de, default=None):
    entry = {"type": kind}
    if default is not None:
        entry["default"] = default
    entry["description"] = {"en": en, "de": de}
    return entry


COMMON = {
    "data": lambda default: option("string", "Name of the source table.", "Name der Quelltabelle.", default),
    "pad": option("number", "Space between the figure frame and the drawing, in millimetres.",
                  "Abstand zwischen Figurenrahmen und Zeichnung in Millimetern.", 3),
    "width": option("number", "Drawing width in millimetres; the enclosing figure supplies it when the key is absent.",
                    "Zeichnungsbreite in Millimetern; ohne den Schlüssel liefert sie die umgebende Figur."),
    "height": option("number", "Drawing height in millimetres; the enclosing figure supplies it when the key is absent.",
                     "Zeichnungshöhe in Millimetern; ohne den Schlüssel liefert sie die umgebende Figur."),
    "labels": option("boolean", "Print the captions.", "Die Beschriftungen zeichnen.", True),
    "scheme": option("string", "Named colour ramp: sequential, diverging or cyclic.",
                     "Benannte Farbrampe: sequenziell, divergierend oder zyklisch.", "sequential"),
    "gutter": option("number", "Space reserved for the row and column captions, in millimetres.",
                     "Für Zeilen- und Spaltenbeschriftungen reservierter Raum in Millimetern."),
    "values": option("boolean", "Print the numeric value of every element.",
                     "Den Zahlenwert jedes Elements ausgeben.", False),
    "mode": lambda choices, default: option(
        "string", "Sub-drawing the family builds: %s." % " | ".join(choices),
        "Teilzeichnung, die die Familie aufbaut: %s." % " | ".join(choices), default),
    "column": lambda en, de, default=None: option("string", en, de, default),
}


def col(en, de, default=None):
    return option("string", en, de, default)


def num(en, de, default=None):
    return option("number", en, de, default)


def integer(en, de, default=None):
    return option("integer", en, de, default)


def flag(en, de, default=None):
    return option("boolean", en, de, default)


FAMILIES = {}

FAMILIES["text-viz"] = {
    "data": COMMON["data"]("demo-text-terms"),
    "term": col("Column holding the token or type.", "Spalte mit dem Token oder Typ.", "term"),
    "count": col("Column holding the frequency.", "Spalte mit der Häufigkeit.", "count"),
    "document": col("Column holding the document key of the term-by-document grid.",
                    "Spalte mit dem Dokumentschlüssel des Term-Dokument-Rasters.", "doc"),
    "position": col("Column holding the occurrence position on the text axis.",
                    "Spalte mit der Fundstelle auf der Textachse.", "pos"),
    "time": col("Column holding the reading window.", "Spalte mit dem Lesefenster.", "t"),
    "series": col("Column holding the topic identity of a long topic table.",
                  "Spalte mit der Themenkennung einer langen Thementabelle.", "topic"),
    "value": col("Column holding the numeric channel of the curve and river layouts.",
                 "Spalte mit dem Zahlenkanal der Kurven- und Fluss-Layouts.", "weight"),
    "layout": COMMON["mode"](["spiral", "grid", "matrix", "lane", "curve", "band", "river"], "spiral"),
    "sort": col("Order of the terms: none, count or term.",
                "Reihenfolge der Terme: none, count oder term.", "count"),
    "sizemin": num("Smallest glyph body height in millimetres.",
                   "Kleinste Glyphenhöhe in Millimetern.", 1.8),
    "sizemax": num("Largest glyph body height in millimetres.",
                   "Größte Glyphenhöhe in Millimetern.", 5.4),
    "growth": num("Archimedean spiral growth per radian, in millimetres.",
                  "Wachstum der archimedischen Spirale je Radiant in Millimetern.", 0.9),
    "step": num("Angular step of the placement spiral, in radians.",
                "Winkelschritt der Platzierungsspirale in Radiant.", 0.35),
    "aspect": num("Horizontal stretch of the placement spiral.",
                  "Waagerechte Streckung der Platzierungsspirale.", 1.7),
    "tries": integer("Placement attempts before a word is dropped.",
                     "Platzierungsversuche, bevor ein Wort entfällt.", 220),
    "columns": integer("Columns of the grid layout.", "Spalten des Rasterlayouts.", 4),
    "collide": flag("Reject a position whose box meets an already placed box.",
                    "Eine Position ablehnen, deren Kasten einen gesetzten Kasten berührt.", True),
    "rotate": flag("Place every second word on its side.",
                   "Jedes zweite Wort hochkant setzen.", False),
    "marker": col("Occurrence glyph of the lane layout: tick, dot or band.",
                  "Fundstellenglyphe des Spurlayouts: tick, dot oder band.", "tick"),
    "context": num("Half width of a keyword-in-context band, in text units.",
                   "Halbe Breite eines Schlüsselwortbandes in Texteinheiten.", 4),
    "guide": flag("Draw the centre guide of the lane layouts.",
                  "Die Mittellinie der Spurlayouts zeichnen.", False),
    "baseline": col("River offset: zero, silhouette or wiggle.",
                    "Flussversatz: zero, silhouette oder wiggle.", "zero"),
    "area": flag("Fill the curve layouts.", "Die Kurvenlayouts füllen.", True),
    "points": flag("Mark every sample of the curve layouts.",
                   "Jeden Messpunkt der Kurvenlayouts markieren.", False),
    "padding": num("Relative gap between adjacent cells and bands.",
                   "Relativer Abstand zwischen benachbarten Zellen und Bändern.", 0.12),
    "pad": COMMON["pad"], "width": COMMON["width"], "height": COMMON["height"],
    "labels": COMMON["labels"],
}

FAMILIES["spatial-layout"] = {
    "data": COMMON["data"]("demo-scatter"),
    "x": col("Column holding the source abscissa.", "Spalte mit der Quellabszisse.", "x"),
    "y": col("Column holding the source ordinate.", "Spalte mit der Quellordinate.", "y"),
    "size": col("Column driving the node area.", "Spalte, die die Knotenfläche steuert.", "size"),
    "label": col("Column holding the node caption.", "Spalte mit der Knotenbeschriftung.", "label"),
    "group": col("Column driving the palette slot.", "Spalte, die den Palettenplatz steuert.", "grp"),
    "algorithm": COMMON["mode"](
        ["beeswarm", "voronoi", "relax", "align", "grid", "matrix", "layered", "radial", "spiral"], "grid"),
    "radius": num("Collision radius handed to the beeswarm layout, in millimetres.",
                  "Kollisionsradius für das Beeswarm-Layout in Millimetern.", 2.4),
    "amount": num("Relaxation step, ring spacing or spiral growth, in millimetres.",
                  "Relaxationsschritt, Ringabstand oder Spiralwachstum in Millimetern.", 2),
    "columns": integer("Columns of the grid and of the small multiples.",
                       "Spalten des Rasters und der Kleinserie.", 5),
    "rings": integer("Rings of the radial layout.", "Ringe des radialen Layouts.", 2),
    "iterations": integer("Relaxation passes of the relax algorithm.",
                          "Relaxationsdurchläufe des Relax-Algorithmus.", 24),
    "node": num("Base node radius in millimetres.", "Grundradius eines Knotens in Millimetern.", 1.1),
    "cells": flag("Draw the polygons or the panel frames.",
                  "Die Polygone oder die Feldrahmen zeichnen.", False),
    "pad": COMMON["pad"], "width": COMMON["width"], "height": COMMON["height"],
    "labels": COMMON["labels"],
}

FAMILIES["performance"] = {
    "data": COMMON["data"]("demo-hierarchy-deep"),
    "id": col("Column holding the frame identity.", "Spalte mit der Rahmenkennung.", "id"),
    "parent": col("Column holding the calling frame, empty at a root.",
                  "Spalte mit dem aufrufenden Rahmen, an einer Wurzel leer.", "parent"),
    "value": col("Column holding the self time.", "Spalte mit der Eigenzeit.", "value"),
    "label": col("Column holding the frame caption.", "Spalte mit der Rahmenbeschriftung.", "label"),
    "source": col("Column holding the calling service of the call graph.",
                  "Spalte mit dem aufrufenden Dienst des Aufrufgraphen.", "id"),
    "target": col("Column holding the called service of the call graph.",
                  "Spalte mit dem aufgerufenen Dienst des Aufrufgraphen.", "target"),
    "mode": COMMON["mode"](["flame", "icicle", "tree", "histogram", "graph"], "flame"),
    "orientation": col("Growth direction of the flame rows: bottom-up or top-down.",
                       "Wuchsrichtung der Flammenzeilen: bottom-up oder top-down.", "bottom-up"),
    "order": col("Sibling order: none, value or label.",
                 "Geschwisterreihenfolge: none, value oder label.", "none"),
    "bins": integer("Bins of the latency histogram.", "Klassen des Latenzhistogramms.", 12),
    "percentiles": option("string", "Comma separated quantiles marked on the latency histogram.",
                          "Kommagetrennte Quantile, die im Latenzhistogramm markiert werden."),
    "rowgap": num("Gap between two depth rows, in millimetres.",
                  "Abstand zwischen zwei Tiefenzeilen in Millimetern.", 0.4),
    "pad": COMMON["pad"], "width": COMMON["width"], "height": COMMON["height"],
    "labels": COMMON["labels"],
}

FAMILIES["schedule"] = {
    "data": COMMON["data"]("demo-sprint"),
    "time": col("Column holding the sprint index.", "Spalte mit dem Sprintindex.", "t"),
    "series": option("string", "Comma separated value columns to draw.",
                     "Kommagetrennte Wertespalten, die gezeichnet werden.", "remaining"),
    "mode": COMMON["mode"](["line", "area", "bar"], "line"),
    "stacked": flag("Accumulate the series on one another.",
                    "Die Reihen aufeinander stapeln.", False),
    "ideal": flag("Draw the ideal burn reference.", "Die ideale Burn-Referenz zeichnen.", False),
    "zero": flag("Draw the zero rule.", "Die Nulllinie zeichnen.", False),
    "gap": num("Gap between the bars of two time steps, in millimetres.",
               "Abstand zwischen den Balken zweier Zeitschritte in Millimetern.", 1.2),
    "pad": COMMON["pad"], "width": COMMON["width"], "height": COMMON["height"],
    "labels": COMMON["labels"],
}

FAMILIES["version-control"] = {
    "data": COMMON["data"]("demo-repo"),
    "time": col("Column holding the commit window.", "Spalte mit dem Commit-Fenster.", "t"),
    "series": option("string", "Comma separated value columns; the added lines first, the removed lines second.",
                     "Kommagetrennte Wertespalten; zuerst die hinzugefügten, dann die entfernten Zeilen.",
                     "additions,deletions"),
    "mode": COMMON["mode"](["line", "area", "bar"], "area"),
    "mirror": flag("Draw the second series below the zero rule.",
                   "Die zweite Reihe unter der Nulllinie zeichnen.", True),
    "zero": flag("Draw the zero rule.", "Die Nulllinie zeichnen.", True),
    "gap": num("Gap between the bars of two windows, in millimetres.",
               "Abstand zwischen den Balken zweier Fenster in Millimetern.", 1.2),
    "pad": COMMON["pad"], "width": COMMON["width"], "height": COMMON["height"],
    "labels": COMMON["labels"],
}

FAMILIES["engineering-diagram"] = {
    "data": COMMON["data"]("demo-machine"),
    "id": col("Column holding the part identity.", "Spalte mit der Teilekennung.", "id"),
    "label": col("Column holding the part caption.", "Spalte mit der Teilebeschriftung.", "label"),
    "class": col("Column holding the part class: body, gear or link.",
                 "Spalte mit der Teileklasse: body, gear oder link.", "kind"),
    "x": col("Column holding the nominal abscissa of the part.",
             "Spalte mit der Sollabszisse des Teils.", "x"),
    "y": col("Column holding the nominal ordinate of the part.",
             "Spalte mit der Sollordinate des Teils.", "y"),
    "size": col("Column holding the body extent in millimetres.",
                "Spalte mit der Körperausdehnung in Millimetern.", "size"),
    "teeth": col("Column holding the tooth count of a gear.",
                 "Spalte mit der Zähnezahl eines Zahnrads.", "teeth"),
    "link": col("Column naming the part this one is jointed to.",
                "Spalte, die das Teil nennt, mit dem dieses gelenkig verbunden ist.", "link"),
    "mode": COMMON["mode"](
        ["assembly", "exploded", "gear", "kinematic", "linkage", "mechanism", "truss"], "assembly"),
    "explode": num("Radial displacement away from the centroid, in millimetres.",
                   "Radiale Verschiebung vom Schwerpunkt weg in Millimetern.", 0),
    "body": num("Scale factor applied to every part extent.",
                "Skalierungsfaktor für jede Teileausdehnung.", 1),
    "node": num("Joint and pin radius in millimetres.",
                "Gelenk- und Bolzenradius in Millimetern.", 0.9),
    "leaders": flag("Draw a dashed leader from the nominal to the displaced place.",
                    "Eine gestrichelte Hilfslinie vom Soll- zum verschobenen Ort zeichnen.", False),
    "ground": flag("Hatch the fixed part.", "Das feste Teil schraffieren.", False),
    "pad": COMMON["pad"], "width": COMMON["width"], "height": COMMON["height"],
    "labels": COMMON["labels"],
}

FAMILIES["optimization"] = {
    "data": COMMON["data"]("demo-assignment"),
    "mode": COMMON["mode"](["assignment", "critical-path", "flow", "transport", "schedule"], "assignment"),
    "row": col("Column holding the agent, the machine or the arc source.",
               "Spalte mit dem Agenten, der Maschine oder der Kantenquelle.", "agent"),
    "column": col("Column holding the task or the arc target.",
                  "Spalte mit der Aufgabe oder dem Kantenziel.", "task"),
    "value": col("Column holding the cost, the flow or the duration.",
                 "Spalte mit den Kosten, dem Fluss oder der Dauer.", "cost"),
    "capacity": col("Column holding the arc capacity.", "Spalte mit der Kantenkapazität.", "capacity"),
    "id": col("Column holding the activity identity.", "Spalte mit der Vorgangskennung.", "agent"),
    "pred": col("Column naming the activity this one waits for.",
                "Spalte, die den Vorgang nennt, auf den dieser wartet.", "pred"),
    "label": col("Column holding the visible caption.", "Spalte mit der sichtbaren Beschriftung.", "label"),
    "start": col("Column holding the start of a scheduled job.",
                 "Spalte mit dem Beginn eines eingeplanten Auftrags.", "start"),
    "scheme": COMMON["scheme"],
    "gutter": num("Space reserved for the row and column captions, in millimetres.",
                  "Für Zeilen- und Spaltenbeschriftungen reservierter Raum in Millimetern.", 6),
    "node": num("Node radius or half height of a bar, in millimetres.",
                "Knotenradius oder halbe Balkenhöhe in Millimetern.", 1.6),
    "arrow": num("Arrow head length in millimetres.", "Länge der Pfeilspitze in Millimetern.", 1.4),
    "values": COMMON["values"],
    "select": flag("Mark the greedy optimum of every row.",
                   "Das gierige Optimum jeder Zeile markieren.", False),
    "pad": COMMON["pad"], "width": COMMON["width"], "height": COMMON["height"],
    "labels": COMMON["labels"],
}

FAMILIES["analytical"] = {
    "data": COMMON["data"]("demo-causes"),
    "cause": col("Column holding the cause itself.", "Spalte mit der Ursache selbst.", "cause"),
    "category": col("Column holding the bone the cause hangs on.",
                    "Spalte mit der Gräte, an der die Ursache hängt.", "category"),
    "effect": option("string", "Caption of the effect box; the bilingual default is printed when it is empty.",
                     "Beschriftung des Wirkungskastens; leer wird die zweisprachige Vorgabe gesetzt."),
    "angle": num("Slope of a bone against the spine, in degrees.",
                 "Neigung einer Gräte gegen die Wirbelsäule in Grad.", 34),
    "bone": num("Length of a bone in millimetres.", "Länge einer Gräte in Millimetern.", 14),
    "twig": num("Length of a cause twig in millimetres.",
                "Länge eines Ursachenzweigs in Millimetern.", 4),
    "pad": COMMON["pad"], "width": COMMON["width"], "height": COMMON["height"],
    "labels": COMMON["labels"],
}

FAMILIES["notation"] = {
    "data": COMMON["data"]("demo-molecule"),
    "mode": COMMON["mode"](["molecule", "circuit", "staff"], "molecule"),
    "id": col("Column holding the symbol identity.", "Spalte mit der Symbolkennung.", "id"),
    "label": col("Column holding the printed glyph name.",
                 "Spalte mit dem gedruckten Glyphennamen.", "label"),
    "class": col("Column holding the component class of a circuit symbol.",
                 "Spalte mit der Bauteilklasse eines Schaltzeichens.", "kind"),
    "x": col("Column holding the abscissa, or the time step of a note.",
             "Spalte mit der Abszisse oder dem Zeitschritt einer Note.", "x"),
    "y": col("Column holding the ordinate, or the staff step of a note.",
             "Spalte mit der Ordinate oder der Notenlinienstufe.", "y"),
    "link": col("Column naming the symbol this one connects to.",
                "Spalte, die das Symbol nennt, mit dem dieses verbunden ist.", "bond"),
    "order": col("Column holding the bond order, or the length of a note.",
                 "Spalte mit der Bindungsordnung oder der Notenlänge.", "order"),
    "node": num("Glyph radius in millimetres.", "Glyphenradius in Millimetern.", 1.4),
    "offset": num("Gap between the two rules of a double bond, in millimetres.",
                  "Abstand zwischen den beiden Strichen einer Doppelbindung in Millimetern.", 0.5),
    "lines": integer("Rules of the staff, and peaks of the resistor glyph.",
                     "Notenlinien und Zacken des Widerstandssymbols.", 5),
    "pad": COMMON["pad"], "width": COMMON["width"], "height": COMMON["height"],
    "labels": COMMON["labels"],
}

FAMILIES["logistics"] = {
    "data": COMMON["data"]("demo-shipments"),
    "mode": COMMON["mode"](["matrix", "sankey"], "matrix"),
    "origin": col("Column holding the shipping depot.", "Spalte mit dem Versanddepot.", "origin"),
    "destination": col("Column holding the receiving depot.", "Spalte mit dem Empfangsdepot.", "destination"),
    "value": col("Column holding the shipped quantity.", "Spalte mit der versandten Menge.", "volume"),
    "scheme": COMMON["scheme"],
    "gutter": num("Space reserved for the depot captions, in millimetres.",
                  "Für die Depotbeschriftungen reservierter Raum in Millimetern.", 8),
    "node": num("Width of an axis node bar, in millimetres.",
                "Breite eines Achsknotenbalkens in Millimetern.", 2),
    "values": COMMON["values"],
    "pad": COMMON["pad"], "width": COMMON["width"], "height": COMMON["height"],
    "labels": COMMON["labels"],
}

FAMILIES["niche"] = {
    "data": COMMON["data"]("demo-survey"),
    "mode": COMMON["mode"](["alluvial", "coxcomb", "icicle", "spiral"], "alluvial"),
    "source": col("Column holding the left category, the part name or the node identity.",
                  "Spalte mit der linken Kategorie, dem Teilenamen oder der Knotenkennung.", "question"),
    "target": col("Column holding the right category.", "Spalte mit der rechten Kategorie.", "group"),
    "value": col("Column holding the quantity.", "Spalte mit der Menge.", "value"),
    "parent": col("Column holding the parent node of the stratified tree.",
                  "Spalte mit dem Elternknoten des geschichteten Baums.", "parent"),
    "label": col("Column holding the visible caption.", "Spalte mit der sichtbaren Beschriftung.", "label"),
    "scheme": COMMON["scheme"],
    "node": num("Axis node width, or the radial band of a spiral cell, in millimetres.",
                "Achsknotenbreite oder radiales Band einer Spiralzelle in Millimetern.", 2),
    "turns": num("Turns of the spiral.", "Windungen der Spirale.", 2.5),
    "gutter": num("Space reserved for the captions, in millimetres.",
                  "Für die Beschriftungen reservierter Raum in Millimetern.", 8),
    "values": COMMON["values"],
    "pad": COMMON["pad"], "width": COMMON["width"], "height": COMMON["height"],
    "labels": COMMON["labels"],
}

FAMILIES["monitoring"] = {
    "data": COMMON["data"]("demo-services"),
    "edges": col("Name of the call table.", "Name der Aufruftabelle.", "demo-services-edges"),
    "mode": COMMON["mode"](["layered", "heat", "graph", "rings"], "layered"),
    "id": col("Column holding the service identity, or the row of the latency grid.",
              "Spalte mit der Dienstkennung oder der Zeile des Latenzrasters.", "id"),
    "column": col("Column holding the column of the latency grid.",
                  "Spalte mit der Spalte des Latenzrasters.", "bucket"),
    "label": col("Column holding the service caption.", "Spalte mit der Dienstbeschriftung.", "label"),
    "tier": col("Column holding the layer the service belongs to.",
                "Spalte mit der Schicht, zu der der Dienst gehört.", "tier"),
    "value": col("Column holding the response time.", "Spalte mit der Antwortzeit.", "latency"),
    "errors": col("Column driving the node tint.", "Spalte, die die Knotenfärbung steuert.", "errors"),
    "source": col("Column holding the calling service.", "Spalte mit dem aufrufenden Dienst.", "source"),
    "target": col("Column holding the called service.", "Spalte mit dem aufgerufenen Dienst.", "target"),
    "weight": col("Column holding the call volume.", "Spalte mit dem Aufrufvolumen.", "calls"),
    "scheme": COMMON["scheme"],
    "node": num("Node radius in millimetres.", "Knotenradius in Millimetern.", 1.6),
    "arrow": num("Arrow head length in millimetres; zero draws plain connectors.",
                 "Länge der Pfeilspitze in Millimetern; null zeichnet schlichte Verbinder.", 0),
    "gutter": num("Space reserved for the captions, in millimetres.",
                  "Für die Beschriftungen reservierter Raum in Millimetern.", 8),
    "pad": COMMON["pad"], "width": COMMON["width"], "height": COMMON["height"],
    "labels": COMMON["labels"],
}

FAMILIES["neural"] = {
    "data": COMMON["data"]("demo-neural"),
    "mode": COMMON["mode"](["single", "multiples"], "single"),
    "channel": col("Column holding the channel of a cell.", "Spalte mit dem Kanal einer Zelle.", "channel"),
    "row": col("Column holding the row of a cell.", "Spalte mit der Zeile einer Zelle.", "row"),
    "column": col("Column holding the column of a cell.", "Spalte mit der Spalte einer Zelle.", "col"),
    "value": col("Column holding the activation.", "Spalte mit der Aktivierung.", "value"),
    "select": integer("Which channel the single mode shows.",
                      "Welchen Kanal der Einzelmodus zeigt.", 1),
    "columns": integer("Channels per row of the small multiples.",
                       "Kanäle je Zeile der Kleinserie.", 2),
    "scheme": COMMON["scheme"],
    "threshold": num("The activation an outlined cell must reach.",
                     "Die Aktivierung, die eine umrandete Zelle erreichen muss.", 0.5),
    "outline": flag("Ring the cells above the threshold.",
                    "Die Zellen oberhalb der Schwelle umranden.", False),
    "gutter": num("Gap between two channel panels, in millimetres.",
                  "Abstand zwischen zwei Kanalfeldern in Millimetern.", 1.5),
    "pad": COMMON["pad"], "width": COMMON["width"], "height": COMMON["height"],
    "labels": flag("Print the channel captions.", "Die Kanalbeschriftungen ausgeben.", False),
}

FAMILIES["survey"] = {
    "data": COMMON["data"]("demo-survey"),
    "question": col("Column holding the row of the grid.", "Spalte mit der Zeile des Rasters.", "question"),
    "group": col("Column holding the column of the grid.", "Spalte mit der Spalte des Rasters.", "group"),
    "value": col("Column holding the share that agrees.", "Spalte mit dem zustimmenden Anteil.", "value"),
    "mark": col("Cell glyph: cell or symbol.", "Zellglyphe: cell oder symbol.", "cell"),
    "scheme": COMMON["scheme"],
    "gutter": num("Space reserved for the captions, in millimetres.",
                  "Für die Beschriftungen reservierter Raum in Millimetern.", 8),
    "node": num("Greatest symbol radius in millimetres.",
                "Größter Symbolradius in Millimetern.", 2.2),
    "values": COMMON["values"],
    "pad": COMMON["pad"], "width": COMMON["width"], "height": COMMON["height"],
    "labels": COMMON["labels"],
}

FAMILIES["election"] = {
    "data": COMMON["data"]("demo-election"),
    "mode": COMMON["mode"](["tile", "cartogram", "swing"], "tile"),
    "label": col("Column holding the constituency caption.", "Spalte mit der Wahlkreisbeschriftung.", "region"),
    "x": col("Column holding the grid abscissa.", "Spalte mit der Rasterabszisse.", "x"),
    "y": col("Column holding the grid ordinate.", "Spalte mit der Rasterordinate.", "y"),
    "value": col("Column holding the tinted quantity.", "Spalte mit der eingefärbten Menge.", "share"),
    "delta": col("Column holding the swing the arrows carry.",
                 "Spalte mit dem Wechsel, den die Pfeile tragen.", "swing"),
    "size": col("Column holding the quantity a cartogram tile sizes.",
                "Spalte mit der Menge, die eine Kartogrammkachel bemisst.", "seats"),
    "scheme": COMMON["scheme"],
    "inset": num("Gap between two tiles, in millimetres.",
                 "Abstand zwischen zwei Kacheln in Millimetern.", 0.6),
    "arrow": num("Arrow head length of a swing arrow, in millimetres.",
                 "Länge der Pfeilspitze eines Wechselpfeils in Millimetern.", 1.2),
    "gain": num("Length of a swing arrow at the greatest swing, in millimetres.",
                "Länge eines Wechselpfeils beim größten Wechsel in Millimetern.", 4),
    "pad": COMMON["pad"], "width": COMMON["width"], "height": COMMON["height"],
    "labels": flag("Print the constituency captions.", "Die Wahlkreisbeschriftungen ausgeben.", False),
}

FAMILIES["biology"] = {
    "data": COMMON["data"]("demo-biodiversity"),
    "mode": COMMON["mode"](["graduated", "range"], "graduated"),
    "label": col("Column holding the plot caption.", "Spalte mit der Flächenbeschriftung.", "cell"),
    "x": col("Column holding the grid abscissa.", "Spalte mit der Rasterabszisse.", "x"),
    "y": col("Column holding the grid ordinate.", "Spalte mit der Rasterordinate.", "y"),
    "value": col("Column holding the species richness.", "Spalte mit dem Artenreichtum.", "richness"),
    "group": col("Column holding the taxon holding the plot.",
                 "Spalte mit dem Taxon, das die Fläche besetzt.", "species"),
    "scheme": COMMON["scheme"],
    "node": num("Greatest symbol radius in millimetres.",
                "Größter Symbolradius in Millimetern.", 2.6),
    "inset": num("Gap between two plots, in millimetres.",
                 "Abstand zwischen zwei Flächen in Millimetern.", 0.6),
    "pad": COMMON["pad"], "width": COMMON["width"], "height": COMMON["height"],
    "labels": flag("Print the plot captions.", "Die Flächenbeschriftungen ausgeben.", False),
}

schema = json.load(io.open(SCHEMA, encoding="utf-8"))


def locate(node):
    if isinstance(node, dict):
        for key, value in node.items():
            if key == "x-semio-family-options":
                return value
            found = locate(value)
            if found is not None:
                return found
    return None


vocabulary = locate(schema)
if vocabulary is None:
    raise SystemExit("x-semio-family-options not found")

for name, options in FAMILIES.items():
    existing = vocabulary.get(name, {})
    variant = existing.get("options", {}).get("variant")
    if variant is None:
        raise SystemExit("no variant option to inherit for " + name)
    merged = {"variant": variant}
    merged.update(options)
    vocabulary[name] = {"owner": OWNER, "options": merged}

tmp = SCHEMA + ".tmp"
with io.open(tmp, "w", encoding="utf-8", newline="\n") as handle:
    json.dump(schema, handle, ensure_ascii=False, indent=2)
    handle.write("\n")
os.replace(tmp, SCHEMA)
print("families written:", len(FAMILIES))
