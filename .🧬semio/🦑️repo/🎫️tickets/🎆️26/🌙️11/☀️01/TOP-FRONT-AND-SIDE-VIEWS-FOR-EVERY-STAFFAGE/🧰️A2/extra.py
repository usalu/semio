GENERAL = (
    "Shapes follow their bones in three dimensions, so a pitched trunk or a lowered head foreshortens in the front view.",
    "Die Formen folgen ihren Knochen dreidimensional, daher verkürzen sich ein geneigter Rumpf oder ein gesenkter Kopf in der Vorderansicht.",
)

SPECIFIC = {
    "mallard-duck": (
        "The wings spread out of the body plane in wing-stretch (abductionDeg about the shoulder, Bone.z beside the symmetry plane): the front wing grows outwards with its bone and the plan wing swings out from the shoulder. The neck is a tapered column that follows the three neck bones.",
        "Die Flügel spreizen sich in wing-stretch aus der Körperebene (abductionDeg um die Schulter, Bone.z neben der Symmetrieebene): der Flügel wächst in der Vorderansicht mit seinem Knochen nach außen, in der Draufsicht schwingt er von der Schulter aus. Der Hals ist eine sich verjüngende Säule, die den drei Halsknochen folgt.",
    ),
    "pigeon": (
        "The wing parts are extruded solids with the outline of the side-view feathers, so the wings spread laterally in the half-open, take-off and airborne poses (abductionDeg about the shoulder, Bone.z beside the symmetry plane): they form a V when raised and droop when lowered. The neck is a tapered column over its three bones and the toes are extruded bars that follow the foot bone.",
        "Die Flügelteile sind extrudierte Körper mit dem Umriss der Federn aus der Seitenansicht, daher spreizen sich die Flügel in den Posen halb geöffnet, Abflug und Flug seitlich (abductionDeg um die Schulter, Bone.z neben der Symmetrieebene): gehoben bilden sie ein V, gesenkt hängen sie. Der Hals ist eine sich verjüngende Säule über drei Knochen, die Zehen sind extrudierte Balken, die dem Fußknochen folgen.",
    ),
    "red-fox": (
        "In curled-sleeping the spine, neck, head and brush bend sideways (abductionDeg, cumulative along each chain), so the plan shows the curled ring and the side view foreshortens; the head cock of mousing-pounce is an axial twist of the head. The brush parts are extruded solids with the outline of the side-view brush.",
        "In curled-sleeping biegen sich Wirbelsäule, Hals, Kopf und Lunte seitlich (abductionDeg, entlang jeder Kette aufsummiert), daher zeigt die Draufsicht den eingerollten Ring und die Seitenansicht verkürzt sich; das Schiefhalten des Kopfes bei mousing-pounce ist eine Axialdrehung des Kopfes. Die Luntenteile sind extrudierte Körper mit dem Umriss der Lunte aus der Seitenansicht.",
    ),
    "red-squirrel": (
        "The tail parts are extruded solids with the outline of the side-view tail.",
        "Die Schwanzteile sind extrudierte Körper mit dem Umriss des Schwanzes aus der Seitenansicht.",
    ),
    "sheep": (
        "The neck is an extruded solid with the outline of the side-view neck.",
        "Der Hals ist ein extrudierter Körper mit dem Umriss des Halses aus der Seitenansicht.",
    ),
    "roe-deer": (
        "The neck is a tapered column that follows the neck bone.",
        "Der Hals ist eine sich verjüngende Säule, die dem Halsknochen folgt.",
    ),
}

SCALE_SPECIFIC = {
    ("red-fox", 200): (
        "The cannon bones carry a rigid pad so they keep their height when the leg lies down, and the plan has a muzzle disc.",
        "Die Röhrbeine tragen eine starre Zugabe, damit sie ihre Höhe behalten, wenn das Bein liegt, und die Draufsicht hat eine Fangscheibe.",
    ),
    ("roe-deer", 500): (
        "Head and ears are drawn as strokes in the front view because a lowered head foreshortens below the print feature size.",
        "Kopf und Ohren sind in der Vorderansicht als Striche gezeichnet, weil ein gesenkter Kopf unter die Druck-Mindestgröße verkürzt.",
    ),
}


OVERRIDE = {
    ("mallard-duck", 200): (
        "The wings spread out of the body plane in wing-stretch (abductionDeg about the shoulder, Bone.z beside the symmetry plane): each wing is one stroke that grows outwards with its bone in the front view and swings out from the shoulder in the plan.",
        "Die Flügel spreizen sich in wing-stretch aus der Körperebene (abductionDeg um die Schulter, Bone.z neben der Symmetrieebene): jeder Flügel ist ein Strich, der in der Vorderansicht mit seinem Knochen nach außen wächst und in der Draufsicht von der Schulter ausschwingt.",
    ),
    ("pigeon", 200): (
        "The wings are two extruded strips with the outline of the side-view wing, so they spread laterally in the half-open, take-off and airborne poses (abductionDeg about the shoulder, Bone.z beside the symmetry plane); the neck is one tapered column.",
        "Die Flügel sind zwei extrudierte Streifen mit dem Umriss des Flügels aus der Seitenansicht und spreizen sich daher in den Posen halb geöffnet, Abflug und Flug seitlich (abductionDeg um die Schulter, Bone.z neben der Symmetrieebene); der Hals ist eine einzelne sich verjüngende Säule.",
    ),
}


def texts(aid, scale, base_en, base_de):
    en, de = [base_en], [base_de]
    for src in (OVERRIDE.get((aid, scale), SPECIFIC.get(aid)), SCALE_SPECIFIC.get((aid, scale)), GENERAL):
        if src is not None:
            en.append(src[0])
            de.append(src[1])
    return " ".join(en), " ".join(de)
