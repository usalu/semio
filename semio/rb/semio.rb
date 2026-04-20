# frozen_string_literal: true

require 'date'

module Semio
  # ✖️ A 3D point (xyz) with floating-point coordinates.
  Point = Struct.new(:x, :y, :z, keyword_init: true)

  # ➡️ A 3D vector (xyz) with floating-point coordinates.
  Vector = Struct.new(:x, :y, :z, keyword_init: true)

  # 📺 A 2D point (uv) in the diagram coordinate system.
  Coordinate = Struct.new(:u, :v, keyword_init: true)

  # � A geographic point with longitude, latitude and optional altitude.
  Location = Struct.new(:longitude, :latitude, keyword_init: true)

  # � Identifier for a type by name and optional variant.
  TypeId = Struct.new(:name, :variant, keyword_init: true)

  # � Identifier for a piece within a design.
  PieceId = Struct.new(:id_, keyword_init: true)

  # 🔌 Identifier for a connector within a type.
  ConnectorId = Struct.new(:id_, keyword_init: true)

  # 💎 A key-value metadata entry with optional definition.
  Attribute = Struct.new(:name, :value, :unit, :definition, keyword_init: true)

  # ✍️ A named contributor with optional email and rank.
  Author = Struct.new(:name, :email, :rank, keyword_init: true)

  # � A 3D representation reference linking a file with tags and description.
  Representation = Struct.new(:url, :description, :tags, :attributes, keyword_init: true)

  # ◻️ A plane defined by an origin point and two axis vectors.
  Plane = Struct.new(:origin, :x_axis, :y_axis, keyword_init: true)

  # 🔌 A connector is a connection point on a type, defined by a point and direction.
  Connector = Struct.new(
    :id_,
    :description,
    :port,
    :mandatory,
    :max_children,
    :t,
    :compatible_ports,
    :point,
    :direction,
    :attributes,
    keyword_init: true
  )

  # 🧩 A positioned instance of a type within a design.
  Piece = Struct.new(:id_, :description, :type, :plane, :center, :attributes, keyword_init: true)

  # ↔️ A side of a connection identifying a specific connector on a piece.
  Side = Struct.new(:piece, :connector, keyword_init: true)

  # � A spatial relationship between two pieces with gap, shift and rotation.
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

  # � A reusable element blueprint with connectors for connection.
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
    :representations,
    :connectors,
    :authors,
    :attributes,
    keyword_init: true
  )

  # 📐 An assembly of pieces, connections, layers and groups.
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

  # 📦 The root container for all domain entities.
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
