"""📐️ The tolerance, the line separator and the IfcOpenShell kernel helpers every audit shares: the placement, the volume and bounds measured by the kernel, the authored records and the comparison helpers."""

from __future__ import annotations

# region 🔖️Imports
import ifcopenshell.geom
import ifcopenshell.util.element
import ifcopenshell.util.shape
# endregion 🔖️Imports


# region 🔖️Measurement
TOLERANCE = 1e-9
NEWLINE = chr(10)
"""⚖️ Absolute tolerance between the kernel's volume, the written quantity and the subject's report."""


def identity(element):
    """🔖️ The model id the export carries in `Tag` (elements and types), `Name` (annotations) or `ObjectType` (spatial elements)."""
    if element.is_a("IfcAnnotation"):
        return element.Name
    return element.Tag if element.is_a("IfcElement") or element.is_a("IfcTypeProduct") else element.ObjectType


def settings():
    """⚙️ Kernel settings: world coordinates, openings subtracted."""
    kernel = ifcopenshell.geom.settings()
    kernel.set("use-world-coords", True)
    return kernel


def kernel_volume(kernel, element):
    """🧊️ The volume of the tessellated element (`None` when it has no swept or faceted body)."""
    try:
        shape = ifcopenshell.geom.create_shape(kernel, element)
    except RuntimeError:
        return None
    geometry = shape.geometry
    return float(ifcopenshell.util.shape.get_volume(geometry))


def kernel_bounds(kernel, element):
    """📦️ The world-space `(min, max)` corners of the tessellated element, or `None`."""
    try:
        shape = ifcopenshell.geom.create_shape(kernel, element)
    except RuntimeError:
        return None
    geometry = shape.geometry
    vertices = ifcopenshell.util.shape.get_vertices(geometry)
    return vertices.min(axis=0), vertices.max(axis=0)


def authored(element):
    """🏷️ The `Semio_Authoring` rows of an element."""
    return ifcopenshell.util.element.get_psets(element).get("Semio_Authoring", {})


def representation_items(element):
    """🧩️ `[(identifier, item)]` of every item of every representation of an element."""
    if element.Representation is None:
        return []
    return [(shape.RepresentationIdentifier, item) for shape in element.Representation.Representations for item in shape.Items]


def record_of(element, name):
    """🏷️ The authored record row `name` of an element, `None` when absent."""
    return authored(element).get(name)


def quantities_of(element, name):
    """🧮️ One base-quantity set of an element, or an empty dict."""
    return ifcopenshell.util.element.get_psets(element, qtos_only=True).get(name, {})


def close(left, right, relative=1e-9):
    """⚖️ Equality within a relative tolerance (absolute below one)."""
    return abs(left - right) <= relative * max(abs(right), 1.0)


# endregion 🔖️Measurement
