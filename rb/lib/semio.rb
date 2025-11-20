# frozen_string_literal: true

require 'date'

module Semio
  # ✖️ A 3D point (xyz) with floating-point coordinates.
  Point = Struct.new(:x, :y, :z, keyword_init: true)

  # ➡️ A 3D vector (xyz) with floating-point coordinates.
  Vector = Struct.new(:x, :y, :z, keyword_init: true)

  # 📺 A 2D point (uv) in the diagram coordinate system.
  Coord = Struct.new(:u, :v, keyword_init: true)

  # 🗺️ The optional location of the type
  Location = Struct.new(:longitude, :latitude, keyword_init: true)

  # 🪪 Identifier for a type, potentially including a variant.
  TypeId = Struct.new(:name, :variant, keyword_init: true)

  # 🪪 Identifier for a piece within a design.
  PieceId = Struct.new(:id_, keyword_init: true)

  # 🪪 Identifier for a port within a type.
  PortId = Struct.new(:id_, keyword_init: true)

  # 🏷️ Represents a attribute, a named property with an optional value, unit, and definition.
  Attribute = Struct.new(:name, :value, :unit, :definition, keyword_init: true)

  # 📑 Represents an author.
  Author = Struct.new(:name, :email, :rank, keyword_init: true)

  # 💾 A model links to a resource (e.g., file) describing a type.
  Model = Struct.new(:url, :description, :tags, :attributes, keyword_init: true)

  # ◳ A plane defined by an origin point and two axes vectors.
  Plane = Struct.new(:origin, :x_axis, :y_axis, keyword_init: true)

  # 🔌 A port is a connection point on a type, defined by a point and direction.
  Port = Struct.new(
    :id_,
    :description,
    :interface,
    :mandatory,
    :t,
    :compatible_interfaces,
    :point,
    :direction,
    :attributes,
    keyword_init: true
  )

  # ⭕ A piece is a 3D instance of a type within a design.
  Piece = Struct.new(:id_, :description, :type, :plane, :center, :attributes, keyword_init: true)

  # 🧱 A side of a piece in a connection, identifying a specific port on a specific piece.
  Side = Struct.new(:piece, :port, keyword_init: true)

  # 🖇️ A bidirectional connection between two pieces of a design.
  Connection = Struct.new(
    :connected,
    :connecting,
    :description,
    :gap,
    :shift,
    :rise,
    :rotation,
    :turn,
    :tilt,
    :x,
    :y,
    :attributes,
    keyword_init: true
  )

  # 🧩 A type is a reusable element blueprint with ports for connection.
  Type = Struct.new(
    :name,
    :description,
    :icon,
    :image,
    :variant,
    :stock,
    :virtual,
    :unit,
    :created,
    :updated,
    :location,
    :models,
    :ports,
    :authors,
    :attributes,
    keyword_init: true
  )

  # 🏙️ A design is a collection of connected pieces.
  Design = Struct.new(
    :name,
    :description,
    :icon,
    :image,
    :variant,
    :view,
    :location,
    :unit,
    :created,
    :updated,
    :pieces,
    :connections,
    :authors,
    :attributes,
    keyword_init: true
  )

  # ↗️ Represents a Kit, the top-level container for types and designs.
  Kit = Struct.new(
    :name,
    :description,
    :icon,
    :image,
    :preview,
    :version,
    :remote,
    :homepage,
    :license,
    :created,
    :updated,
    :types,
    :designs,
    :attributes,
    keyword_init: true
  )
end
