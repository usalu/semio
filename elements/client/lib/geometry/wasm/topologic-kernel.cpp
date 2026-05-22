#include <cmath>
#include <cstddef>
#include <string>
#include <unordered_set>
#include <vector>

#include <emscripten/bind.h>
#include <emscripten/val.h>

namespace {

using emscripten::val;

constexpr const char* kSchema = "elements.geometry.topologic.fixture/v1";
constexpr const char* kKinds[] = {
	"topology",
	"vertex",
	"edge",
	"wire",
	"face",
	"shell",
	"cell",
	"cellComplex",
	"cluster",
};

bool is_array(const val& value) {
	return val::global("Array").call<bool>("isArray", value);
}

bool is_record(const val& value) {
	return !value.isUndefined() && !value.isNull() && value.typeOf().as<std::string>() == "object" && !is_array(value);
}

bool is_nonempty_string(const val& value) {
	return !value.isUndefined() && !value.isNull() && value.typeOf().as<std::string>() == "string" && !value.as<std::string>().empty();
}

bool is_finite_number(const val& value) {
	return !value.isUndefined() && !value.isNull() && value.typeOf().as<std::string>() == "number" && std::isfinite(value.as<double>());
}

std::size_t array_length(const val& value) {
	return value["length"].as<std::size_t>();
}

bool is_known_kind(const std::string& kind) {
	for (const char* current : kKinds) {
		if (kind == current) return true;
	}
	return false;
}

val clone_vec3(const val& value) {
	if (!is_array(value) || array_length(value) != 3) return val::null();
	val out = val::array();
	for (std::size_t index = 0; index < 3; index += 1) {
		const val entry = value[index];
		if (!is_finite_number(entry)) return val::null();
		out.call<void>("push", entry.as<double>());
	}
	return out;
}

val clone_quat(const val& value) {
	if (!is_array(value) || array_length(value) != 4) return val::null();
	val out = val::array();
	for (std::size_t index = 0; index < 4; index += 1) {
		const val entry = value[index];
		if (!is_finite_number(entry)) return val::null();
		out.call<void>("push", entry.as<double>());
	}
	return out;
}

val clone_string_array(const val& value) {
	if (!is_array(value)) return val::null();
	val out = val::array();
	for (std::size_t index = 0; index < array_length(value); index += 1) {
		const val entry = value[index];
		if (!is_nonempty_string(entry)) return val::null();
		out.call<void>("push", entry.as<std::string>());
	}
	return out;
}

val clone_style(const val& value) {
	if (!is_record(value)) return val::undefined();
	val out = val::object();
	bool has_any = false;
	const val color = value["color"];
	const val edge_color = value["edgeColor"];
	const val opacity = value["opacity"];
	const val line_width = value["lineWidth"];
	const val point_size = value["pointSize"];
	if (is_nonempty_string(color)) {
		out.set("color", color.as<std::string>());
		has_any = true;
	}
	if (is_nonempty_string(edge_color)) {
		out.set("edgeColor", edge_color.as<std::string>());
		has_any = true;
	}
	if (is_finite_number(opacity)) {
		out.set("opacity", opacity.as<double>());
		has_any = true;
	}
	if (is_finite_number(line_width)) {
		out.set("lineWidth", line_width.as<double>());
		has_any = true;
	}
	if (is_finite_number(point_size)) {
		out.set("pointSize", point_size.as<double>());
		has_any = true;
	}
	return has_any ? out : val::undefined();
}

val clone_transform(const val& value) {
	if (!is_record(value)) return val::undefined();
	val out = val::object();
	bool has_any = false;
	const val position = clone_vec3(value["position"]);
	const val rotation = clone_quat(value["rotation"]);
	if (!position.isNull()) {
		out.set("position", position);
		has_any = true;
	}
	if (!rotation.isNull()) {
		out.set("rotation", rotation);
		has_any = true;
	}
	const val scale = value["scale"];
	if (is_finite_number(scale)) {
		out.set("scale", scale.as<double>());
		has_any = true;
	} else {
		const val scale_vec = clone_vec3(scale);
		if (!scale_vec.isNull()) {
			out.set("scale", scale_vec);
			has_any = true;
		}
	}
	return has_any ? out : val::undefined();
}

val clone_base(const val& raw) {
	if (!is_record(raw) || !is_nonempty_string(raw["id"])) return val::null();
	val out = val::object();
	out.set("id", raw["id"].as<std::string>());
	if (is_nonempty_string(raw["label"])) out.set("label", raw["label"].as<std::string>());
	if (is_nonempty_string(raw["description"])) out.set("description", raw["description"].as<std::string>());
	const val style = clone_style(raw["style"]);
	const val transform = clone_transform(raw["transform"]);
	if (!style.isUndefined()) out.set("style", style);
	if (!transform.isUndefined()) out.set("transform", transform);
	if (is_record(raw["metadata"])) out.set("metadata", raw["metadata"]);
	return out;
}

val clone_surface(const val& value) {
	if (!is_record(value) || !is_array(value["vertices"]) || !is_array(value["triangles"])) return val::null();
	val out = val::object();
	val vertices = val::array();
	for (std::size_t index = 0; index < array_length(value["vertices"]); index += 1) {
		const val point = clone_vec3(value["vertices"][index]);
		if (point.isNull()) return val::null();
		vertices.call<void>("push", point);
	}
	val triangles = val::array();
	for (std::size_t index = 0; index < array_length(value["triangles"]); index += 1) {
		const val entry = value["triangles"][index];
		if (!is_finite_number(entry)) return val::null();
		const double number = entry.as<double>();
		if (std::floor(number) != number) return val::null();
		triangles.call<void>("push", static_cast<int>(number));
	}
	out.set("vertices", vertices);
	out.set("triangles", triangles);
	return out;
}

val clone_entity(const val& raw) {
	if (!is_record(raw) || !is_nonempty_string(raw["kind"])) return val::null();
	const std::string kind = raw["kind"].as<std::string>();
	if (!is_known_kind(kind)) return val::null();
	val base = clone_base(raw);
	if (base.isNull()) return val::null();
	base.set("kind", kind);
	if (kind == "topology") {
		const val members = clone_string_array(raw["members"]);
		if (members.isNull()) return val::null();
		base.set("members", members);
		return base;
	}
	if (kind == "vertex") {
		const val point = clone_vec3(raw["point"]);
		if (point.isNull()) return val::null();
		base.set("point", point);
		if (is_finite_number(raw["radius"])) base.set("radius", raw["radius"].as<double>());
		return base;
	}
	if (kind == "edge") {
		const val vertices = clone_string_array(raw["vertices"]);
		if (vertices.isNull() || array_length(vertices) != 2) return val::null();
		base.set("vertices", vertices);
		if (is_array(raw["curve"])) {
			val curve = val::array();
			for (std::size_t index = 0; index < array_length(raw["curve"]); index += 1) {
				const val point = clone_vec3(raw["curve"][index]);
				if (point.isNull()) return val::null();
				curve.call<void>("push", point);
			}
			base.set("curve", curve);
		}
		return base;
	}
	if (kind == "wire") {
		const val edges = clone_string_array(raw["edges"]);
		if (edges.isNull()) return val::null();
		base.set("edges", edges);
		if (raw["closed"].strictlyEquals(val(true))) base.set("closed", true);
		base.set("manifold", raw["manifold"].strictlyEquals(val(false)) ? false : true);
		return base;
	}
	if (kind == "face") {
		const val wires = clone_string_array(raw["wires"]);
		const val surface = clone_surface(raw["surface"]);
		if (wires.isNull() || surface.isNull()) return val::null();
		base.set("wires", wires);
		base.set("surface", surface);
		return base;
	}
	if (kind == "shell") {
		const val faces = clone_string_array(raw["faces"]);
		if (faces.isNull()) return val::null();
		base.set("faces", faces);
		return base;
	}
	if (kind == "cell") {
		const val shells = clone_string_array(raw["shells"]);
		if (shells.isNull()) return val::null();
		base.set("shells", shells);
		return base;
	}
	if (kind == "cellComplex") {
		const val cells = clone_string_array(raw["cells"]);
		if (cells.isNull()) return val::null();
		base.set("cells", cells);
		return base;
	}
	const val topologies = clone_string_array(raw["topologies"]);
	if (topologies.isNull()) return val::null();
	base.set("topologies", topologies);
	return base;
}

std::vector<std::string> child_ids(const val& entity) {
	const std::string kind = entity["kind"].as<std::string>();
	val ids = val::array();
	if (kind == "topology") ids = entity["members"];
	else if (kind == "wire") ids = entity["edges"];
	else if (kind == "face") ids = entity["wires"];
	else if (kind == "shell") ids = entity["faces"];
	else if (kind == "cell") ids = entity["shells"];
	else if (kind == "cellComplex") ids = entity["cells"];
	else if (kind == "cluster") ids = entity["topologies"];
	else if (kind == "edge") ids = entity["vertices"];
	std::vector<std::string> out;
	for (std::size_t index = 0; index < array_length(ids); index += 1) {
		out.push_back(ids[index].as<std::string>());
	}
	return out;
}

val find_entity(const val& fixture, const std::string& entity_id) {
	const val topologies = fixture["topologies"];
	for (std::size_t index = 0; index < array_length(topologies); index += 1) {
		val entity = topologies[index];
		if (entity["id"].as<std::string>() == entity_id) return entity;
	}
	return val::undefined();
}

val normalize_transform(const val& transform) {
	val out = val::object();
	const val position = clone_vec3(transform["position"]);
	const val rotation = clone_quat(transform["rotation"]);
	out.set("position", position.isNull() ? val::array() : position);
	if (position.isNull()) {
		val fallback = val::array();
		fallback.call<void>("push", 0.0);
		fallback.call<void>("push", 0.0);
		fallback.call<void>("push", 0.0);
		out.set("position", fallback);
	}
	if (rotation.isNull()) {
		val fallback = val::array();
		fallback.call<void>("push", 0.0);
		fallback.call<void>("push", 0.0);
		fallback.call<void>("push", 0.0);
		fallback.call<void>("push", 1.0);
		out.set("rotation", fallback);
	} else {
		out.set("rotation", rotation);
	}
	const val scale = transform["scale"];
	if (is_finite_number(scale)) out.set("scale", scale.as<double>());
	else {
		const val scale_vec = clone_vec3(scale);
		if (scale_vec.isNull()) out.set("scale", 1.0);
		else out.set("scale", scale_vec);
	}
	return out;
}

val transform_point(const val& point, const val& transform) {
	const val parsed_point = clone_vec3(point);
	if (parsed_point.isNull()) return val::null();
	const val normalized = normalize_transform(transform);
	const val scale = normalized["scale"];
	const double sx = scale.typeOf().as<std::string>() == "number" ? scale.as<double>() : scale[0].as<double>();
	const double sy = scale.typeOf().as<std::string>() == "number" ? scale.as<double>() : scale[1].as<double>();
	const double sz = scale.typeOf().as<std::string>() == "number" ? scale.as<double>() : scale[2].as<double>();
	const double px = normalized["position"][0].as<double>();
	const double py = normalized["position"][1].as<double>();
	const double pz = normalized["position"][2].as<double>();
	const double qx = normalized["rotation"][0].as<double>();
	const double qy = normalized["rotation"][1].as<double>();
	const double qz = normalized["rotation"][2].as<double>();
	const double qw = normalized["rotation"][3].as<double>();
	const double x = parsed_point[0].as<double>() * sx;
	const double y = parsed_point[1].as<double>() * sy;
	const double z = parsed_point[2].as<double>() * sz;
	const double ix = qw * x + qy * z - qz * y;
	const double iy = qw * y + qz * x - qx * z;
	const double iz = qw * z + qx * y - qy * x;
	const double iw = -qx * x - qy * y - qz * z;
	val out = val::array();
	out.call<void>("push", ix * qw + iw * -qx + iy * -qz - iz * -qy + px);
	out.call<void>("push", iy * qw + iw * -qy + iz * -qx - ix * -qz + py);
	out.call<void>("push", iz * qw + iw * -qz + ix * -qy - iy * -qx + pz);
	return out;
}

val parse_fixture(const val& raw) {
	if (!is_record(raw) || !raw["schema"].strictlyEquals(val(kSchema))) return val::null();
	const val roots = clone_string_array(raw["roots"]);
	if (roots.isNull() || !is_array(raw["topologies"])) return val::null();
	val topologies = val::array();
	std::unordered_set<std::string> entity_ids;
	for (std::size_t index = 0; index < array_length(raw["topologies"]); index += 1) {
		const val entity = clone_entity(raw["topologies"][index]);
		if (entity.isNull()) return val::null();
		const std::string entity_id = entity["id"].as<std::string>();
		if (entity_ids.contains(entity_id)) return val::null();
		entity_ids.insert(entity_id);
		topologies.call<void>("push", entity);
	}
	for (std::size_t index = 0; index < array_length(roots); index += 1) {
		if (!entity_ids.contains(roots[index].as<std::string>())) return val::null();
	}
	for (std::size_t index = 0; index < array_length(topologies); index += 1) {
		const val entity = topologies[index];
		for (const std::string& child_id : child_ids(entity)) {
			if (!entity_ids.contains(child_id)) return val::null();
		}
		if (entity["kind"].as<std::string>() == "face") {
			const val surface = entity["surface"];
			const std::size_t vertex_count = array_length(surface["vertices"]);
			const std::size_t triangle_count = array_length(surface["triangles"]);
			if (triangle_count % 3 != 0) return val::null();
			for (std::size_t triangle_index = 0; triangle_index < triangle_count; triangle_index += 1) {
				const int current = surface["triangles"][triangle_index].as<int>();
				if (current < 0 || static_cast<std::size_t>(current) >= vertex_count) return val::null();
			}
		}
	}
	val fixture = val::object();
	fixture.set("schema", kSchema);
	if (is_nonempty_string(raw["label"])) fixture.set("label", raw["label"].as<std::string>());
	fixture.set("roots", roots);
	fixture.set("topologies", topologies);
	return fixture;
}

val vertex_point(const val& fixture, const std::string& entity_id) {
	const val entity = find_entity(fixture, entity_id);
	if (entity.isUndefined() || entity["kind"].as<std::string>() != "vertex") return val::null();
	return transform_point(entity["point"], entity["transform"]);
}

val edge_curve(const val& fixture, const std::string& entity_id) {
	const val entity = find_entity(fixture, entity_id);
	if (entity.isUndefined() || entity["kind"].as<std::string>() != "edge") return val::array();
	if (is_array(entity["curve"]) && array_length(entity["curve"]) >= 2) return entity["curve"];
	const val start = vertex_point(fixture, entity["vertices"][0].as<std::string>());
	const val end = vertex_point(fixture, entity["vertices"][1].as<std::string>());
	if (start.isNull() || end.isNull()) return val::array();
	val out = val::array();
	out.call<void>("push", start);
	out.call<void>("push", end);
	return out;
}

val update_fixture_transform(const val& fixture, const std::string& entity_id, const val& transform) {
	val parsed = parse_fixture(fixture);
	if (parsed.isNull()) return val::null();
	const val next_transform = clone_transform(transform);
	val topologies = parsed["topologies"];
	for (std::size_t index = 0; index < array_length(topologies); index += 1) {
		val entity = topologies[index];
		if (entity["id"].as<std::string>() != entity_id) continue;
		entity.set("transform", next_transform.isUndefined() ? val::object() : next_transform);
		break;
	}
	return parsed;
}

} // namespace

EMSCRIPTEN_BINDINGS(topologic_kernel) {
	emscripten::function("parseFixture", &parse_fixture);
	emscripten::function("vertexPoint", &vertex_point);
	emscripten::function("edgeCurve", &edge_curve);
	emscripten::function("updateFixtureTransform", &update_fixture_transform);
}