from lib import *

B = 'beam'
ALL = ['beam-sides', 'beam-top', 'beam-soffit']
views = {}
dec = {}

views['500'] = {
    'front': [rect('beam-end', B, 'body', 1, -0.08, 0.0, 0.16, 0.62, 0.4, regions=ALL + ['bearing-steel'], material='glulam')],
    'plan': [rect('beam-top', B, 'body', 1, 0.0, -0.08, 8.0, 0.16, 0.4, regions=['beam-top'], material='glulam')],
}
dec['500'] = {
    'silhouette': (
        'Front view: the end face of the bar as one filled strip (0.32 x 1.24 mm). Plan view: the top of the bar as one filled strip (16 x 0.32 mm).',
        'Vorderansicht: die Stirnfläche des Balkens als ein gefüllter Streifen (0,32 x 1,24 mm). Draufsicht: die Oberseite des Balkens als ein gefüllter Streifen (16 x 0,32 mm).'),
    'retained': (
        'Width 0.16 m in both views; in the front view the bearing plate level stays inside the strip.',
        'Breite 0,16 m in beiden Ansichten; in der Vorderansicht bleibt die Lagerplattenlage im Streifen.'),
    'omitted': (
        'In plan the bearing plates lie below the bar and are not drawn; the end face has no lamellae lines.',
        'In der Draufsicht liegen die Lagerplatten unter dem Balken und werden nicht gezeichnet; die Stirnfläche trägt keine Lamellenlinien.'),
}

views['200'] = {
    'front': [
        rect('beam-end', B, 'body', 1, -0.08, 0.02, 0.16, 0.6, 0.6, regions=ALL, material='glulam'),
        line('bearing-end', B, 'body', 2, [(-0.08, 0.01), (0.08, 0.01)], 0.3, region='bearing-steel', material='steel', pen=0.18),
    ],
    'plan': [
        rect('beam-top', B, 'body', 1, 0.0, -0.08, 8.0, 0.16, 0.6, regions=['beam-top'], material='glulam'),
        line('beam-axis', B, 'detail', 1.5, [(0.12, 0.0), (7.88, 0.0)], 0.35, region='beam-top'),
    ],
}
dec['200'] = {
    'silhouette': (
        'Front view: the 0.16 x 0.60 m end face (0.8 x 3 mm) over a bearing line. Plan view: the top of the bar (40 x 0.8 mm) with a faint axis line.',
        'Vorderansicht: die Stirnfläche 0,16 x 0,60 m (0,8 x 3 mm) über einer Auflagerlinie. Draufsicht: die Oberseite des Balkens (40 x 0,8 mm) mit einer feinen Achslinie.'),
    'retained': (
        'The plate level as a 0.18 mm line under the end face, as in the side view.',
        'Die Plattenlage als 0,18-mm-Linie unter der Stirnfläche, wie in der Seitenansicht.'),
    'omitted': (
        'Lamination of the end face (0.2 mm spacing) and, in plan, the bearing plates below the bar.',
        'Lamellen der Stirnfläche (0,2 mm Abstand) und in der Draufsicht die Lagerplatten unter dem Balken.'),
}

views['100'] = {
    'front': [
        rect('beam-end', B, 'body', 1, -0.08, 0.02, 0.16, 0.6, 0.6, regions=ALL, material='glulam', hatch='grain'),
        rect('bearing-plate-end', B, 'body', 2, -0.08, -0.005, 0.16, 0.025, 0.3, region='bearing-steel', material='steel', pen=0.1),
    ],
    'plan': [
        rect('beam-top', B, 'body', 1, 0.0, -0.08, 8.0, 0.16, 0.6, regions=['beam-top'], material='glulam', hatch='grain'),
    ],
}
dec['100'] = {
    'silhouette': (
        'Front view: the end face (1.6 x 6 mm) with the glue-line hatch of the lamellae over a bearing plate of the same width. Plan view: the top lamella (80 x 1.6 mm) with the grain hatch along the beam.',
        'Vorderansicht: die Stirnfläche (1,6 x 6 mm) mit der Leimfugenschraffur der Lamellen über einer Lagerplatte gleicher Breite. Draufsicht: die oberste Lamelle (80 x 1,6 mm) mit der Faserschraffur längs des Balkens.'),
    'retained': (
        'The grain hatch at 0.8 mm serves as glue lines in the end face and as grain lines on the top; the plate is 0.16 x 0.025 m.',
        'Die Faserschraffur mit 0,8 mm dient an der Stirnfläche als Leimfugen und an der Oberseite als Faserlinien; die Platte misst 0,16 x 0,025 m.'),
    'omitted': (
        'Finger joints of the top lamella; the bearing plates are hidden below the bar in plan.',
        'Keilzinkenstöße der obersten Lamelle; die Lagerplatten liegen in der Draufsicht verdeckt unter dem Balken.'),
}

lam = [line('glue-line-%02d' % (k + 1), B, 'texture', 1.5, [(-0.08, 0.06 + 0.04 * k), (0.08, 0.06 + 0.04 * k)], 0.3, region='beam-sides') for k in range(14)]
chamfer = [(-0.074, 0.02), (0.074, 0.02), (0.08, 0.026), (0.08, 0.614), (0.074, 0.62), (-0.074, 0.62), (-0.08, 0.614), (-0.08, 0.026)]
cham_plan = [(0.006, -0.08), (7.994, -0.08), (8.0, -0.074), (8.0, 0.074), (7.994, 0.08), (0.006, 0.08), (0.0, 0.074), (0.0, -0.074)]
joints = []
for n, x in enumerate([2.6, 5.9]):
    pts = [(x + (0.02 if i % 2 else 0.0), -0.06 + 0.02 * i) for i in range(7)]
    joints.append(line('finger-joint-top-%d' % n, B, 'detail', 1.6, pts, 0.3, region='beam-top'))
views['50'] = {
    'front': [
        poly('beam-chamfered-end', B, 'body', 1, chamfer, 0.6, regions=ALL, material='glulam'),
        *lam,
        rect('bearing-plate-end', B, 'body', 2, -0.08, 0.0, 0.16, 0.02, 0.3, region='bearing-steel', material='steel'),
    ],
    'plan': [
        poly('beam-chamfered-top', B, 'body', 1, cham_plan, 0.6, regions=['beam-top'], material='glulam'),
        *joints,
    ],
}
dec['50'] = {
    'silhouette': (
        'Front view: the end face of 0.16 x 0.60 m (3.2 x 12 mm) with chamfered edges, fifteen lamellae separated by fourteen glue lines and the 20 mm bearing plate below. Plan view: the top lamella (160 x 3.2 mm) with chamfered corners and two staggered finger joints.',
        'Vorderansicht: die Stirnfläche 0,16 x 0,60 m (3,2 x 12 mm) mit gefasten Kanten, fünfzehn Lamellen, getrennt durch vierzehn Leimfugen, und die 20-mm-Lagerplatte darunter. Draufsicht: die oberste Lamelle (160 x 3,2 mm) mit gefasten Ecken und zwei versetzten Keilzinkenstößen.'),
    'retained': (
        'All 14 glue lines at 40 mm in the end face; in plan the 6 mm chamfers at the corners and the finger joints of the top lamella at 2.6 m and 5.9 m.',
        'Alle 14 Leimfugen im Abstand von 40 mm in der Stirnfläche; in der Draufsicht die 6-mm-Fasen an den Ecken und die Keilzinkenstöße der obersten Lamelle bei 2,6 m und 5,9 m.'),
    'omitted': (
        'Plate bolts (not visible from the end) and, in plan, the bearing plates and the chamfer lines along the long edges (6 mm, 0.12 mm, inside the outline pen).',
        'Lagerschrauben (von der Stirnseite nicht sichtbar) und in der Draufsicht die Lagerplatten sowie die Fasenlinien an den Längskanten (6 mm, 0,12 mm, innerhalb der Konturfeder).'),
}

if __name__ == '__main__':
    apply('glulam-beam', views, dec)
