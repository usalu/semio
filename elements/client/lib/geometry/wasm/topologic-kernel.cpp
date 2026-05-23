#include <algorithm>
#include <cmath>
#include <cstddef>
#include <cctype>
#include <string>
#include <unordered_map>
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
	if (!is_record(transform)) {
		val position = val::array();
		position.call<void>("push", 0.0);
		position.call<void>("push", 0.0);
		position.call<void>("push", 0.0);
		val rotation = val::array();
		rotation.call<void>("push", 0.0);
		rotation.call<void>("push", 0.0);
		rotation.call<void>("push", 0.0);
		rotation.call<void>("push", 1.0);
		out.set("position", position);
		out.set("rotation", rotation);
		out.set("scale", 1.0);
		return out;
	}
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

struct AnalyzePoint3 {
	double x;
	double y;
	double z;
};

struct AnalyzeBounds {
	AnalyzePoint3 min;
	AnalyzePoint3 max;
};

struct AnalyzeCellBoundsInfo {
	std::string cell_id;
	std::string label;
	AnalyzeBounds bounds;
	bool overlaps;
};

struct AnalyzeVoxelCell {
	int x_index;
	int y_index;
	int z_index;
	AnalyzeBounds bounds;
	std::vector<std::string> owner_ids;
	std::string owner_key;
	std::string overlap;
};

struct AnalyzeGridPartition {
	std::vector<double> xs;
	std::vector<double> ys;
	std::vector<double> zs;
	std::vector<AnalyzeVoxelCell> voxels;
	std::unordered_map<std::string, std::size_t> voxel_index_by_key;
};

struct AnalyzeVoxelComponent {
	std::string component_id;
	std::vector<std::string> owner_ids;
	std::string overlap;
	std::unordered_set<std::string> voxel_keys;
};

struct AnalyzeMergedGridRect {
	std::string tag;
	int u_start;
	int u_end;
	int v_start;
	int v_end;
};

struct AnalyzeFaceRectInfo {
	std::string cell_id;
	std::string face_id;
	std::string label;
	char axis;
	double plane;
	double u0;
	double u1;
	double v0;
	double v1;
	bool positive_side;
	std::string kind;
};

bool starts_with(const std::string& value, const std::string& prefix) {
	return value.rfind(prefix, 0) == 0;
}

AnalyzePoint3 read_analyze_point3(const val& value) {
	return { value[0].as<double>(), value[1].as<double>(), value[2].as<double>() };
}

val make_analyze_point3(const AnalyzePoint3& point) {
	val out = val::array();
	out.call<void>("push", point.x);
	out.call<void>("push", point.y);
	out.call<void>("push", point.z);
	return out;
}

val make_string_array(const std::vector<std::string>& values) {
	val out = val::array();
	for (const std::string& value : values) out.call<void>("push", value);
	return out;
}

std::string join_strings(const std::vector<std::string>& values, const std::string& separator) {
	std::string out;
	for (std::size_t index = 0; index < values.size(); index += 1) {
		if (index > 0) out += separator;
		out += values[index];
	}
	return out;
}

std::vector<std::string> split_string(const std::string& value, const char separator) {
	std::vector<std::string> out;
	std::string current;
	for (const char character : value) {
		if (character == separator) {
			out.push_back(current);
			current.clear();
			continue;
		}
		current.push_back(character);
	}
	out.push_back(current);
	return out;
}

std::string capitalize(const std::string& value) {
	if (value.empty()) return value;
	std::string out = value;
	out[0] = static_cast<char>(std::toupper(static_cast<unsigned char>(out[0])));
	return out;
}

std::string humanize_analyze_kind(const std::string& kind) {
	if (kind == "solid") return "Solid";
	const std::vector<std::string> parts = split_string(kind, '.');
	if (parts.size() == 3 && parts[0] == "surface") return capitalize(parts[1]) + " " + capitalize(parts[2]);
	if (parts.size() == 2 && parts[0] == "part") return capitalize(parts[1]);
	return kind;
}

val style_for_analyze_kind(const std::string& kind) {
	val style = val::object();
	if (kind == "surface.external.horizontal") {
		style.set("color", std::string("#38bdf8"));
		style.set("edgeColor", std::string("#7dd3fc"));
		style.set("opacity", 0.4);
	} else if (kind == "surface.external.vertical") {
		style.set("color", std::string("#60a5fa"));
		style.set("edgeColor", std::string("#93c5fd"));
		style.set("opacity", 0.28);
	} else if (kind == "surface.internal.horizontal") {
		style.set("color", std::string("#f59e0b"));
		style.set("edgeColor", std::string("#fbbf24"));
		style.set("opacity", 0.42);
	} else if (kind == "surface.internal.vertical") {
		style.set("color", std::string("#f97316"));
		style.set("edgeColor", std::string("#fdba74"));
		style.set("opacity", 0.34);
	} else if (kind == "part.none") {
		style.set("color", std::string("#22c55e"));
		style.set("edgeColor", std::string("#4ade80"));
		style.set("opacity", 0.22);
	} else if (kind == "part.difference") {
		style.set("color", std::string("#eab308"));
		style.set("edgeColor", std::string("#facc15"));
		style.set("opacity", 0.24);
	} else if (kind == "part.intersection") {
		style.set("color", std::string("#ef4444"));
		style.set("edgeColor", std::string("#f87171"));
		style.set("opacity", 0.34);
	} else {
		style.set("color", std::string("#a855f7"));
		style.set("edgeColor", std::string("#d8b4fe"));
		style.set("opacity", 0.14);
	}
	return style;
}

val analyze_metadata(const std::string& kind, const bool selectable, const std::string& parent_id = "") {
	val metadata = val::object();
	metadata.set("analyzeKind", kind);
	metadata.set("analyzeGroup", starts_with(kind, "surface.") ? std::string("surface") : starts_with(kind, "part.") ? std::string("part") : std::string("solid"));
	metadata.set("analyzeSelectable", selectable);
	if (!parent_id.empty()) metadata.set("analyzeParentId", parent_id);
	return metadata;
}

std::string analyze_voxel_key(const int x_index, const int y_index, const int z_index) {
	return std::to_string(x_index) + ":" + std::to_string(y_index) + ":" + std::to_string(z_index);
}

void collect_entity_points(const val& fixture, const std::string& entity_id, std::vector<AnalyzePoint3>& points) {
	const val entity = find_entity(fixture, entity_id);
	if (entity.isUndefined()) return;
	const std::string kind = entity["kind"].as<std::string>();
	if (kind == "face") {
		const val vertices = entity["surface"]["vertices"];
		for (std::size_t index = 0; index < array_length(vertices); index += 1) points.push_back(read_analyze_point3(vertices[index]));
	}
	if (kind == "vertex") points.push_back(read_analyze_point3(entity["point"]));
	for (const std::string& child_id : child_ids(entity)) collect_entity_points(fixture, child_id, points);
}

AnalyzeBounds create_analyze_bounds(const std::vector<AnalyzePoint3>& points) {
	AnalyzeBounds bounds = {
		{ points.front().x, points.front().y, points.front().z },
		{ points.front().x, points.front().y, points.front().z },
	};
	for (const AnalyzePoint3& point : points) {
		bounds.min.x = std::min(bounds.min.x, point.x);
		bounds.min.y = std::min(bounds.min.y, point.y);
		bounds.min.z = std::min(bounds.min.z, point.z);
		bounds.max.x = std::max(bounds.max.x, point.x);
		bounds.max.y = std::max(bounds.max.y, point.y);
		bounds.max.z = std::max(bounds.max.z, point.z);
	}
	return bounds;
}

bool analyze_bounds_contains_point(const AnalyzeBounds& bounds, const AnalyzePoint3& point, const double epsilon = 1e-9) {
	return point.x >= bounds.min.x - epsilon && point.x <= bounds.max.x + epsilon && point.y >= bounds.min.y - epsilon && point.y <= bounds.max.y + epsilon && point.z >= bounds.min.z - epsilon && point.z <= bounds.max.z + epsilon;
}

bool analyze_bounds_overlap(const AnalyzeBounds& left, const AnalyzeBounds& right, const double epsilon = 1e-9) {
	return std::min(left.max.x, right.max.x) - std::max(left.min.x, right.min.x) > epsilon && std::min(left.max.y, right.max.y) - std::max(left.min.y, right.min.y) > epsilon && std::min(left.max.z, right.max.z) - std::max(left.min.z, right.min.z) > epsilon;
}

std::vector<double> unique_sorted(std::vector<double> values) {
	std::sort(values.begin(), values.end());
	values.erase(std::unique(values.begin(), values.end()), values.end());
	return values;
}

double interval_center(const double start, const double end) {
	return (start + end) / 2.0;
}

std::vector<AnalyzeCellBoundsInfo> collect_cell_bounds(const val& fixture) {
	std::vector<AnalyzeCellBoundsInfo> cells;
	const val topologies = fixture["topologies"];
	for (std::size_t index = 0; index < array_length(topologies); index += 1) {
		const val entity = topologies[index];
		if (entity["kind"].as<std::string>() != "cell") continue;
		std::vector<AnalyzePoint3> points;
		collect_entity_points(fixture, entity["id"].as<std::string>(), points);
		if (points.empty()) continue;
		cells.push_back({ entity["id"].as<std::string>(), is_nonempty_string(entity["label"]) ? entity["label"].as<std::string>() : entity["id"].as<std::string>(), create_analyze_bounds(points), false });
	}
	for (std::size_t left = 0; left < cells.size(); left += 1) {
		for (std::size_t right = left + 1; right < cells.size(); right += 1) {
			if (!analyze_bounds_overlap(cells[left].bounds, cells[right].bounds)) continue;
			cells[left].overlaps = true;
			cells[right].overlaps = true;
		}
	}
	return cells;
}

AnalyzeGridPartition create_grid_partition(const std::vector<AnalyzeCellBoundsInfo>& cells) {
	AnalyzeGridPartition partition;
	for (const AnalyzeCellBoundsInfo& cell : cells) {
		partition.xs.push_back(cell.bounds.min.x);
		partition.xs.push_back(cell.bounds.max.x);
		partition.ys.push_back(cell.bounds.min.y);
		partition.ys.push_back(cell.bounds.max.y);
		partition.zs.push_back(cell.bounds.min.z);
		partition.zs.push_back(cell.bounds.max.z);
	}
	partition.xs = unique_sorted(partition.xs);
	partition.ys = unique_sorted(partition.ys);
	partition.zs = unique_sorted(partition.zs);
	for (std::size_t x_index = 0; x_index + 1 < partition.xs.size(); x_index += 1) {
		for (std::size_t y_index = 0; y_index + 1 < partition.ys.size(); y_index += 1) {
			for (std::size_t z_index = 0; z_index + 1 < partition.zs.size(); z_index += 1) {
				AnalyzeBounds bounds = {
					{ partition.xs[x_index], partition.ys[y_index], partition.zs[z_index] },
					{ partition.xs[x_index + 1], partition.ys[y_index + 1], partition.zs[z_index + 1] },
				};
				const AnalyzePoint3 center = { interval_center(bounds.min.x, bounds.max.x), interval_center(bounds.min.y, bounds.max.y), interval_center(bounds.min.z, bounds.max.z) };
				std::vector<std::string> owners;
				bool owner_overlaps = false;
				for (const AnalyzeCellBoundsInfo& cell : cells) {
					if (!analyze_bounds_contains_point(cell.bounds, center)) continue;
					owners.push_back(cell.cell_id);
					owner_overlaps = cell.overlaps;
				}
				if (owners.empty()) continue;
				const std::string owner_key = join_strings(owners, "|");
				const std::string overlap = owners.size() > 1 ? "intersection" : owner_overlaps ? "difference" : "none";
				partition.voxels.push_back({ static_cast<int>(x_index), static_cast<int>(y_index), static_cast<int>(z_index), bounds, owners, owner_key, overlap });
				partition.voxel_index_by_key[analyze_voxel_key(static_cast<int>(x_index), static_cast<int>(y_index), static_cast<int>(z_index))] = partition.voxels.size() - 1;
			}
		}
	}
	return partition;
}

const AnalyzeVoxelCell* find_analyze_voxel(const AnalyzeGridPartition& partition, const int x_index, const int y_index, const int z_index) {
	const auto iterator = partition.voxel_index_by_key.find(analyze_voxel_key(x_index, y_index, z_index));
	if (iterator == partition.voxel_index_by_key.end()) return nullptr;
	return &partition.voxels[iterator->second];
}

std::vector<AnalyzeVoxelComponent> collect_voxel_components(const AnalyzeGridPartition& partition) {
	std::vector<AnalyzeVoxelComponent> components;
	std::unordered_set<std::string> visited;
	for (const AnalyzeVoxelCell& seed : partition.voxels) {
		const std::string seed_key = analyze_voxel_key(seed.x_index, seed.y_index, seed.z_index);
		if (visited.contains(seed_key)) continue;
		std::vector<const AnalyzeVoxelCell*> stack = { &seed };
		visited.insert(seed_key);
		AnalyzeVoxelComponent component = { "analyze.part." + std::to_string(components.size() + 1), seed.owner_ids, seed.overlap, {} };
		while (!stack.empty()) {
			const AnalyzeVoxelCell* current = stack.back();
			stack.pop_back();
			component.voxel_keys.insert(analyze_voxel_key(current->x_index, current->y_index, current->z_index));
			const int neighbors[6][3] = {
				{ current->x_index - 1, current->y_index, current->z_index },
				{ current->x_index + 1, current->y_index, current->z_index },
				{ current->x_index, current->y_index - 1, current->z_index },
				{ current->x_index, current->y_index + 1, current->z_index },
				{ current->x_index, current->y_index, current->z_index - 1 },
				{ current->x_index, current->y_index, current->z_index + 1 },
			};
			for (const auto& neighbor : neighbors) {
				const AnalyzeVoxelCell* next = find_analyze_voxel(partition, neighbor[0], neighbor[1], neighbor[2]);
				if (next == nullptr || next->owner_key != seed.owner_key || next->overlap != seed.overlap) continue;
				const std::string next_key = analyze_voxel_key(next->x_index, next->y_index, next->z_index);
				if (visited.contains(next_key)) continue;
				visited.insert(next_key);
				stack.push_back(next);
			}
		}
		components.push_back(component);
	}
	return components;
}

std::vector<AnalyzeMergedGridRect> merge_tagged_grid(const std::vector<std::vector<std::string>>& cells) {
	std::vector<AnalyzeMergedGridRect> rects;
	if (cells.empty()) return rects;
	const int width = static_cast<int>(cells.size());
	const int height = static_cast<int>(cells.front().size());
	std::unordered_set<std::string> visited;
	for (int u_index = 0; u_index < width; u_index += 1) {
		for (int v_index = 0; v_index < height; v_index += 1) {
			const std::string& tag = cells[static_cast<std::size_t>(u_index)][static_cast<std::size_t>(v_index)];
			if (tag.empty()) continue;
			const std::string key = std::to_string(u_index) + ":" + std::to_string(v_index);
			if (visited.contains(key)) continue;
			int u_end = u_index + 1;
			while (u_end < width && cells[static_cast<std::size_t>(u_end)][static_cast<std::size_t>(v_index)] == tag && !visited.contains(std::to_string(u_end) + ":" + std::to_string(v_index))) u_end += 1;
			int v_end = v_index + 1;
			while (v_end < height) {
				bool matches = true;
				for (int scan = u_index; scan < u_end; scan += 1) {
					if (cells[static_cast<std::size_t>(scan)][static_cast<std::size_t>(v_end)] != tag || visited.contains(std::to_string(scan) + ":" + std::to_string(v_end))) {
						matches = false;
						break;
					}
				}
				if (!matches) break;
				v_end += 1;
			}
			for (int mark_u = u_index; mark_u < u_end; mark_u += 1) {
				for (int mark_v = v_index; mark_v < v_end; mark_v += 1) visited.insert(std::to_string(mark_u) + ":" + std::to_string(mark_v));
			}
			rects.push_back({ tag, u_index, u_end, v_index, v_end });
		}
	}
	return rects;
}

val create_rectangle_face(const std::string& id, const std::string& label, const char axis, const double plane, const double u0, const double u1, const double v0, const double v1, const std::string& kind, const bool selectable, const std::string& parent_id = "") {
	val face = val::object();
	face.set("id", id);
	face.set("kind", std::string("face"));
	face.set("label", label);
	face.set("wires", val::array());
	val vertices = val::array();
	if (axis == 'x') {
		vertices.call<void>("push", make_analyze_point3({ plane, u0, v0 }));
		vertices.call<void>("push", make_analyze_point3({ plane, u1, v0 }));
		vertices.call<void>("push", make_analyze_point3({ plane, u1, v1 }));
		vertices.call<void>("push", make_analyze_point3({ plane, u0, v1 }));
	} else if (axis == 'y') {
		vertices.call<void>("push", make_analyze_point3({ u0, plane, v0 }));
		vertices.call<void>("push", make_analyze_point3({ u1, plane, v0 }));
		vertices.call<void>("push", make_analyze_point3({ u1, plane, v1 }));
		vertices.call<void>("push", make_analyze_point3({ u0, plane, v1 }));
	} else {
		vertices.call<void>("push", make_analyze_point3({ u0, v0, plane }));
		vertices.call<void>("push", make_analyze_point3({ u1, v0, plane }));
		vertices.call<void>("push", make_analyze_point3({ u1, v1, plane }));
		vertices.call<void>("push", make_analyze_point3({ u0, v1, plane }));
	}
	val triangles = val::array();
	triangles.call<void>("push", 0);
	triangles.call<void>("push", 1);
	triangles.call<void>("push", 2);
	triangles.call<void>("push", 0);
	triangles.call<void>("push", 2);
	triangles.call<void>("push", 3);
	val surface = val::object();
	surface.set("vertices", vertices);
	surface.set("triangles", triangles);
	face.set("surface", surface);
	face.set("style", style_for_analyze_kind(kind));
	face.set("metadata", analyze_metadata(kind, selectable, parent_id));
	return face;
}

std::vector<val> create_box_faces(const std::string& prefix, const std::string& label, const AnalyzeBounds& bounds, const std::string& kind, const bool selectable, const std::string& parent_id = "") {
	return {
		create_rectangle_face(prefix + ".bottom", label + " Bottom", 'y', bounds.min.y, bounds.min.x, bounds.max.x, bounds.min.z, bounds.max.z, kind, selectable, parent_id),
		create_rectangle_face(prefix + ".top", label + " Top", 'y', bounds.max.y, bounds.min.x, bounds.max.x, bounds.min.z, bounds.max.z, kind, selectable, parent_id),
		create_rectangle_face(prefix + ".left", label + " Left", 'x', bounds.min.x, bounds.min.y, bounds.max.y, bounds.min.z, bounds.max.z, kind, selectable, parent_id),
		create_rectangle_face(prefix + ".right", label + " Right", 'x', bounds.max.x, bounds.min.y, bounds.max.y, bounds.min.z, bounds.max.z, kind, selectable, parent_id),
		create_rectangle_face(prefix + ".front", label + " Front", 'z', bounds.min.z, bounds.min.x, bounds.max.x, bounds.min.y, bounds.max.y, kind, selectable, parent_id),
		create_rectangle_face(prefix + ".back", label + " Back", 'z', bounds.max.z, bounds.min.x, bounds.max.x, bounds.min.y, bounds.max.y, kind, selectable, parent_id),
	};
}

bool nearly_equal(const double left, const double right, const double epsilon = 1e-9) {
	return std::abs(left - right) <= epsilon;
}

bool intervals_overlap(const double left_start, const double left_end, const double right_start, const double right_end, const double epsilon = 1e-9) {
	return std::min(left_end, right_end) - std::max(left_start, right_start) > epsilon;
}

double axis_min(const AnalyzeBounds& bounds, const char axis) {
	if (axis == 'x') return bounds.min.x;
	if (axis == 'y') return bounds.min.y;
	return bounds.min.z;
}

double axis_max(const AnalyzeBounds& bounds, const char axis) {
	if (axis == 'x') return bounds.max.x;
	if (axis == 'y') return bounds.max.y;
	return bounds.max.z;
}

std::pair<double, double> face_projection_u(const AnalyzeBounds& bounds, const char axis) {
	if (axis == 'x') return { bounds.min.y, bounds.max.y };
	return { bounds.min.x, bounds.max.x };
}

std::pair<double, double> face_projection_v(const AnalyzeBounds& bounds, const char axis) {
	if (axis == 'x' || axis == 'y') return { bounds.min.z, bounds.max.z };
	return { bounds.min.y, bounds.max.y };
}

AnalyzeFaceRectInfo create_face_rect_info(const AnalyzeCellBoundsInfo& cell, const val& face) {
	const val vertices = face["surface"]["vertices"];
	double min_x = vertices[0][0].as<double>();
	double max_x = min_x;
	double min_y = vertices[0][1].as<double>();
	double max_y = min_y;
	double min_z = vertices[0][2].as<double>();
	double max_z = min_z;
	for (std::size_t index = 1; index < array_length(vertices); index += 1) {
		const double x = vertices[index][0].as<double>();
		const double y = vertices[index][1].as<double>();
		const double z = vertices[index][2].as<double>();
		min_x = std::min(min_x, x);
		max_x = std::max(max_x, x);
		min_y = std::min(min_y, y);
		max_y = std::max(max_y, y);
		min_z = std::min(min_z, z);
		max_z = std::max(max_z, z);
	}
	AnalyzeFaceRectInfo out = {
		cell.cell_id,
		face["id"].as<std::string>(),
		is_nonempty_string(face["label"]) ? face["label"].as<std::string>() : face["id"].as<std::string>(),
		'z',
		min_z,
		min_x,
		max_x,
		min_y,
		max_y,
		false,
		std::string(),
	};
	if (nearly_equal(min_x, max_x)) {
		out.axis = 'x';
		out.plane = min_x;
		out.u0 = min_y;
		out.u1 = max_y;
		out.v0 = min_z;
		out.v1 = max_z;
	} else if (nearly_equal(min_y, max_y)) {
		out.axis = 'y';
		out.plane = min_y;
		out.u0 = min_x;
		out.u1 = max_x;
		out.v0 = min_z;
		out.v1 = max_z;
	} else {
		out.axis = 'z';
		out.plane = min_z;
		out.u0 = min_x;
		out.u1 = max_x;
		out.v0 = min_y;
		out.v1 = max_y;
	}
	const double min_plane = axis_min(cell.bounds, out.axis);
	const double max_plane = axis_max(cell.bounds, out.axis);
	out.positive_side = nearly_equal(out.plane, max_plane) || std::abs(out.plane - max_plane) < std::abs(out.plane - min_plane);
	return out;
}

std::vector<AnalyzeFaceRectInfo> collect_cell_face_rects(const val& fixture, const std::vector<AnalyzeCellBoundsInfo>& cells) {
	std::vector<AnalyzeFaceRectInfo> faces;
	for (const AnalyzeCellBoundsInfo& cell : cells) {
		const val cell_entity = find_entity(fixture, cell.cell_id);
		if (cell_entity.isUndefined()) continue;
		for (std::size_t shell_index = 0; shell_index < array_length(cell_entity["shells"]); shell_index += 1) {
			const val shell = find_entity(fixture, cell_entity["shells"][shell_index].as<std::string>());
			if (shell.isUndefined()) continue;
			for (std::size_t face_index = 0; face_index < array_length(shell["faces"]); face_index += 1) {
				const val face = find_entity(fixture, shell["faces"][face_index].as<std::string>());
				if (face.isUndefined()) continue;
				faces.push_back(create_face_rect_info(cell, face));
			}
		}
	}
	return faces;
}

bool face_patch_is_internal(const AnalyzeFaceRectInfo& face, const AnalyzeCellBoundsInfo& other, const double u, const double v, const double epsilon = 1e-9) {
	const auto [other_u0, other_u1] = face_projection_u(other.bounds, face.axis);
	const auto [other_v0, other_v1] = face_projection_v(other.bounds, face.axis);
	if (u < other_u0 + epsilon || u > other_u1 - epsilon || v < other_v0 + epsilon || v > other_v1 - epsilon) return false;
	const double other_min = axis_min(other.bounds, face.axis);
	const double other_max = axis_max(other.bounds, face.axis);
	if (face.positive_side) return other_min < face.plane + epsilon && other_max > face.plane + epsilon;
	return other_min < face.plane - epsilon && other_max > face.plane - epsilon;
}

std::vector<AnalyzeFaceRectInfo> split_semantic_surface_patches(const val& fixture, const std::vector<AnalyzeCellBoundsInfo>& cells) {
	std::vector<AnalyzeFaceRectInfo> patches;
	const std::vector<AnalyzeFaceRectInfo> faces = collect_cell_face_rects(fixture, cells);
	for (const AnalyzeFaceRectInfo& face : faces) {
		std::vector<double> us = { face.u0, face.u1 };
		std::vector<double> vs = { face.v0, face.v1 };
		for (const AnalyzeCellBoundsInfo& other : cells) {
			if (other.cell_id == face.cell_id) continue;
			const auto [other_u0, other_u1] = face_projection_u(other.bounds, face.axis);
			const auto [other_v0, other_v1] = face_projection_v(other.bounds, face.axis);
			if (!intervals_overlap(face.u0, face.u1, other_u0, other_u1) || !intervals_overlap(face.v0, face.v1, other_v0, other_v1)) continue;
			const double other_min = axis_min(other.bounds, face.axis);
			const double other_max = axis_max(other.bounds, face.axis);
			const bool reaches_plane = face.positive_side ? other_min < face.plane + 1e-9 && other_max > face.plane + 1e-9 : other_min < face.plane - 1e-9 && other_max > face.plane - 1e-9;
			if (!reaches_plane) continue;
			us.push_back(std::max(face.u0, other_u0));
			us.push_back(std::min(face.u1, other_u1));
			vs.push_back(std::max(face.v0, other_v0));
			vs.push_back(std::min(face.v1, other_v1));
		}
		us = unique_sorted(us);
		vs = unique_sorted(vs);
		for (std::size_t u_index = 0; u_index + 1 < us.size(); u_index += 1) {
			for (std::size_t v_index = 0; v_index + 1 < vs.size(); v_index += 1) {
				const double patch_u0 = us[u_index];
				const double patch_u1 = us[u_index + 1];
				const double patch_v0 = vs[v_index];
				const double patch_v1 = vs[v_index + 1];
				if (patch_u1 - patch_u0 <= 1e-9 || patch_v1 - patch_v0 <= 1e-9) continue;
				const double center_u = interval_center(patch_u0, patch_u1);
				const double center_v = interval_center(patch_v0, patch_v1);
				bool internal = false;
				for (const AnalyzeCellBoundsInfo& other : cells) {
					if (other.cell_id == face.cell_id) continue;
					if (face_patch_is_internal(face, other, center_u, center_v)) {
						internal = true;
						break;
					}
				}
				patches.push_back({
					face.cell_id,
					face.face_id,
					face.label,
					face.axis,
					face.plane,
					patch_u0,
					patch_u1,
					patch_v0,
					patch_v1,
					face.positive_side,
					std::string("surface.") + (internal ? "internal" : "external") + (face.axis == 'y' ? ".horizontal" : ".vertical"),
				});
			}
		}
	}
	return patches;
}

std::vector<AnalyzeFaceRectInfo> merge_surface_patches(const std::vector<AnalyzeFaceRectInfo>& patches) {
	std::unordered_map<std::string, std::vector<AnalyzeFaceRectInfo>> groups;
	for (const AnalyzeFaceRectInfo& patch : patches) groups[std::string(1, patch.axis) + ":" + std::to_string(patch.plane) + ":" + patch.kind].push_back(patch);
	std::vector<AnalyzeFaceRectInfo> merged;
	for (const auto& [key, group] : groups) {
		std::vector<double> us;
		std::vector<double> vs;
		for (const AnalyzeFaceRectInfo& patch : group) {
			us.push_back(patch.u0);
			us.push_back(patch.u1);
			vs.push_back(patch.v0);
			vs.push_back(patch.v1);
		}
		us = unique_sorted(us);
		vs = unique_sorted(vs);
		std::vector<std::vector<std::string>> grid(us.size() > 0 ? us.size() - 1 : 0, std::vector<std::string>(vs.size() > 0 ? vs.size() - 1 : 0, std::string()));
		for (std::size_t u_index = 0; u_index + 1 < us.size(); u_index += 1) {
			for (std::size_t v_index = 0; v_index + 1 < vs.size(); v_index += 1) {
				const double center_u = interval_center(us[u_index], us[u_index + 1]);
				const double center_v = interval_center(vs[v_index], vs[v_index + 1]);
				for (const AnalyzeFaceRectInfo& patch : group) {
					if (center_u <= patch.u0 + 1e-9 || center_u >= patch.u1 - 1e-9 || center_v <= patch.v0 + 1e-9 || center_v >= patch.v1 - 1e-9) continue;
					grid[u_index][v_index] = patch.kind;
					break;
				}
			}
		}
		for (const AnalyzeMergedGridRect& rect : merge_tagged_grid(grid)) {
			merged.push_back({
				group.front().cell_id,
				group.front().face_id,
				group.front().label,
				group.front().axis,
				group.front().plane,
				us[rect.u_start],
				us[rect.u_end],
				vs[rect.v_start],
				vs[rect.v_end],
				group.front().positive_side,
				group.front().kind,
			});
		}
	}
	return merged;
}

val clone_analyze_face(const val& source_face, const std::string& id, const std::string& label, const std::string& kind, const bool selectable, const std::string& parent_id = "") {
	val face = val::object();
	face.set("id", id);
	face.set("kind", std::string("face"));
	face.set("label", label);
	face.set("wires", val::array());
	face.set("surface", clone_surface(source_face["surface"]));
	const val transform = clone_transform(source_face["transform"]);
	if (!transform.isUndefined()) face.set("transform", transform);
	face.set("style", style_for_analyze_kind(kind));
	face.set("metadata", analyze_metadata(kind, selectable, parent_id));
	return face;
}

std::vector<val> create_component_faces(const AnalyzeVoxelComponent& component, const AnalyzeGridPartition& partition, const std::string& label, const std::string& kind) {
	std::vector<val> faces;
	auto component_has_voxel = [&component](const int x_index, const int y_index, const int z_index) {
		return component.voxel_keys.contains(analyze_voxel_key(x_index, y_index, z_index));
	};
	for (int plane_index = 0; plane_index < static_cast<int>(partition.xs.size()); plane_index += 1) {
		std::vector<std::vector<std::string>> grid(partition.ys.size() > 0 ? partition.ys.size() - 1 : 0, std::vector<std::string>(partition.zs.size() > 0 ? partition.zs.size() - 1 : 0, std::string()));
		for (int y_index = 0; y_index < static_cast<int>(partition.ys.size()) - 1; y_index += 1) {
			for (int z_index = 0; z_index < static_cast<int>(partition.zs.size()) - 1; z_index += 1) {
				const bool left = plane_index > 0 ? component_has_voxel(plane_index - 1, y_index, z_index) : false;
				const bool right = plane_index < static_cast<int>(partition.xs.size()) - 1 ? component_has_voxel(plane_index, y_index, z_index) : false;
				if (left != right) grid[static_cast<std::size_t>(y_index)][static_cast<std::size_t>(z_index)] = "face";
			}
		}
		for (const AnalyzeMergedGridRect& rect : merge_tagged_grid(grid)) faces.push_back(create_rectangle_face(component.component_id + ".face." + std::to_string(faces.size() + 1), label + " Face " + std::to_string(faces.size() + 1), 'x', partition.xs[plane_index], partition.ys[rect.u_start], partition.ys[rect.u_end], partition.zs[rect.v_start], partition.zs[rect.v_end], kind, false, component.component_id));
	}
	for (int plane_index = 0; plane_index < static_cast<int>(partition.ys.size()); plane_index += 1) {
		std::vector<std::vector<std::string>> grid(partition.xs.size() > 0 ? partition.xs.size() - 1 : 0, std::vector<std::string>(partition.zs.size() > 0 ? partition.zs.size() - 1 : 0, std::string()));
		for (int x_index = 0; x_index < static_cast<int>(partition.xs.size()) - 1; x_index += 1) {
			for (int z_index = 0; z_index < static_cast<int>(partition.zs.size()) - 1; z_index += 1) {
				const bool bottom = plane_index > 0 ? component_has_voxel(x_index, plane_index - 1, z_index) : false;
				const bool top = plane_index < static_cast<int>(partition.ys.size()) - 1 ? component_has_voxel(x_index, plane_index, z_index) : false;
				if (bottom != top) grid[static_cast<std::size_t>(x_index)][static_cast<std::size_t>(z_index)] = "face";
			}
		}
		for (const AnalyzeMergedGridRect& rect : merge_tagged_grid(grid)) faces.push_back(create_rectangle_face(component.component_id + ".face." + std::to_string(faces.size() + 1), label + " Face " + std::to_string(faces.size() + 1), 'y', partition.ys[plane_index], partition.xs[rect.u_start], partition.xs[rect.u_end], partition.zs[rect.v_start], partition.zs[rect.v_end], kind, false, component.component_id));
	}
	for (int plane_index = 0; plane_index < static_cast<int>(partition.zs.size()); plane_index += 1) {
		std::vector<std::vector<std::string>> grid(partition.xs.size() > 0 ? partition.xs.size() - 1 : 0, std::vector<std::string>(partition.ys.size() > 0 ? partition.ys.size() - 1 : 0, std::string()));
		for (int x_index = 0; x_index < static_cast<int>(partition.xs.size()) - 1; x_index += 1) {
			for (int y_index = 0; y_index < static_cast<int>(partition.ys.size()) - 1; y_index += 1) {
				const bool front = plane_index > 0 ? component_has_voxel(x_index, y_index, plane_index - 1) : false;
				const bool back = plane_index < static_cast<int>(partition.zs.size()) - 1 ? component_has_voxel(x_index, y_index, plane_index) : false;
				if (front != back) grid[static_cast<std::size_t>(x_index)][static_cast<std::size_t>(y_index)] = "face";
			}
		}
		for (const AnalyzeMergedGridRect& rect : merge_tagged_grid(grid)) faces.push_back(create_rectangle_face(component.component_id + ".face." + std::to_string(faces.size() + 1), label + " Face " + std::to_string(faces.size() + 1), 'z', partition.zs[plane_index], partition.xs[rect.u_start], partition.xs[rect.u_end], partition.ys[rect.v_start], partition.ys[rect.v_end], kind, false, component.component_id));
	}
	return faces;
}

val derive_analyze_fixture(const val& fixture) {
	const val parsed = parse_fixture(fixture);
	if (parsed.isNull()) return val::null();
	const std::vector<AnalyzeCellBoundsInfo> cells = collect_cell_bounds(parsed);
	const AnalyzeGridPartition partition = create_grid_partition(cells);
	const std::vector<AnalyzeVoxelComponent> components = collect_voxel_components(partition);
	const std::vector<AnalyzeFaceRectInfo> surface_patches = merge_surface_patches(split_semantic_surface_patches(parsed, cells));
	val topologies = val::array();
	std::vector<std::string> root_members;
	int surface_count = 0;
	int none_count = 0;
	int difference_count = 0;
	int intersection_count = 0;
	for (const AnalyzeFaceRectInfo& patch : surface_patches) {
		surface_count += 1;
		const std::string id = "analyze.surface." + std::to_string(surface_count);
		const std::string label = "Surface " + humanize_analyze_kind(patch.kind) + " " + std::to_string(surface_count);
		topologies.call<void>("push", create_rectangle_face(id, label, patch.axis, patch.plane, patch.u0, patch.u1, patch.v0, patch.v1, patch.kind, true));
		root_members.push_back(id);
	}
	for (const AnalyzeCellBoundsInfo& cell : cells) {
		const std::string solid_id = "analyze.solid." + cell.cell_id;
		const std::string label = "Solid " + cell.label;
		val solid = val::object();
		solid.set("id", solid_id);
		solid.set("kind", std::string("topology"));
		solid.set("label", label);
		std::vector<std::string> member_ids;
		const val cell_entity = find_entity(parsed, cell.cell_id);
		for (std::size_t shell_index = 0; shell_index < array_length(cell_entity["shells"]); shell_index += 1) {
			const val shell = find_entity(parsed, cell_entity["shells"][shell_index].as<std::string>());
			if (shell.isUndefined()) continue;
			for (std::size_t face_index = 0; face_index < array_length(shell["faces"]); face_index += 1) {
				const std::string source_face_id = shell["faces"][face_index].as<std::string>();
				const val source_face = find_entity(parsed, source_face_id);
				if (source_face.isUndefined()) continue;
				const std::string cloned_face_id = solid_id + ".face." + source_face_id;
				const std::string cloned_face_label = label + " " + (is_nonempty_string(source_face["label"]) ? source_face["label"].as<std::string>() : source_face_id);
				topologies.call<void>("push", clone_analyze_face(source_face, cloned_face_id, cloned_face_label, "solid", false, solid_id));
				member_ids.push_back(cloned_face_id);
			}
		}
		solid.set("members", make_string_array(member_ids));
		solid.set("style", style_for_analyze_kind("solid"));
		solid.set("metadata", analyze_metadata("solid", true));
		topologies.call<void>("push", solid);
		root_members.push_back(solid_id);
	}
	for (const AnalyzeVoxelComponent& component : components) {
		int label_count = 0;
		if (component.overlap == "none") label_count = ++none_count;
		else if (component.overlap == "difference") label_count = ++difference_count;
		else label_count = ++intersection_count;
		const std::string kind = "part." + component.overlap;
		const std::string label = "Part " + humanize_analyze_kind(kind) + " " + std::to_string(label_count);
		const std::vector<val> faces = create_component_faces(component, partition, label, kind);
		val part = val::object();
		part.set("id", component.component_id);
		part.set("kind", std::string("topology"));
		part.set("label", label);
		std::vector<std::string> member_ids;
		for (const val& face : faces) member_ids.push_back(face["id"].as<std::string>());
		part.set("members", make_string_array(member_ids));
		part.set("style", style_for_analyze_kind(kind));
		part.set("metadata", analyze_metadata(kind, true));
		topologies.call<void>("push", part);
		for (const val& face : faces) topologies.call<void>("push", face);
		root_members.push_back(component.component_id);
	}
	val out = val::object();
	out.set("schema", std::string(kSchema));
	if (is_nonempty_string(parsed["label"])) out.set("label", parsed["label"].as<std::string>() + std::string(" Analyze"));
	val roots = val::array();
	roots.call<void>("push", std::string("analyze-root"));
	out.set("roots", roots);
	val root = val::object();
	root.set("id", std::string("analyze-root"));
	root.set("kind", std::string("topology"));
	root.set("label", std::string("Analyze Root"));
	root.set("members", make_string_array(root_members));
	val root_metadata = val::object();
	root_metadata.set("analyzeMode", std::string("analyze"));
	root.set("metadata", root_metadata);
	val next_topologies = val::array();
	next_topologies.call<void>("push", root);
	for (std::size_t index = 0; index < array_length(topologies); index += 1) next_topologies.call<void>("push", topologies[index]);
	out.set("topologies", next_topologies);
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
	emscripten::function("deriveAnalyzeFixture", &derive_analyze_fixture);
	emscripten::function("vertexPoint", &vertex_point);
	emscripten::function("edgeCurve", &edge_curve);
	emscripten::function("updateFixtureTransform", &update_fixture_transform);
}