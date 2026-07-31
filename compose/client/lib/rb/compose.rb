# frozen_string_literal: true

require 'date'
require 'securerandom'

module Compose
  # ✖️ A 3D point (xyz) with floating-point coordinates.
  Point = Struct.new(:x, :y, :z, keyword_init: true)

  # ➡️ A 3D vector (xyz) with floating-point coordinates.
  Vector = Struct.new(:x, :y, :z, keyword_init: true)

  # 📺️ A 2D point (uv) in the diagram coordinate system.
  Coordinate = Struct.new(:u, :v, keyword_init: true)

  # �️ A geographic point with longitude, latitude and optional altitude.
  Location = Struct.new(:longitude, :latitude, keyword_init: true)

  # �️ Identifier for a type by name and optional variant.
  TypeId = Struct.new(:name, :variant, keyword_init: true)

  # �️ Identifier for a piece within a design.
  PieceId = Struct.new(:id_, keyword_init: true)

  # 🔌️ Identifier for a connector within a type.
  ConnectorId = Struct.new(:id_, keyword_init: true)

  # 💎️ A key-value metadata entry with optional definition.
  Attribute = Struct.new(:name, :value, :unit, :definition, keyword_init: true)

  # ✍️ A named contributor with optional email and rank.
  Author = Struct.new(:name, :email, :rank, keyword_init: true)

  # �️ A 3D representation reference linking a file with tags and description.
  Representation = Struct.new(:url, :description, :tags, :attributes, keyword_init: true)

  # ◻ A plane defined by an origin point and two axis vectors.
  Plane = Struct.new(:origin, :x_axis, :y_axis, keyword_init: true)

  # 🔌️ A connector is a connection point on a type, defined by a point and direction.
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

  # 🏛️ Identifier for a typology that owns types and designs.
  TypologyId = Struct.new(:id_, keyword_init: true)

  # 🏛️ A kit partition owning types and designs; families stay at kit root.
  Typology = Struct.new(
    :id_,
    :name,
    :description,
    :icon,
    :folder,
    :types,
    :designs,
    keyword_init: true
  )

  # 🧩️ A positioned instance of a type within a design.
  Piece = Struct.new(:id_, :description, :type, :plane, :center, :attributes, keyword_init: true)

  # ↔ A side of a connection identifying a specific connector on a piece.
  Side = Struct.new(:piece, :connector, keyword_init: true)

  # �️ A spatial relationship between two pieces with gap, shift and rotation.
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

  # �️ A reusable element blueprint with connectors for connection.
  Type = Struct.new(
    :name,
    :description,
    :icon,
    :image,
    :variant,
    :stock,
    :virtual,
    :unit,
    :typology,
    :created,
    :updated,
    :location,
    :representations,
    :connectors,
    :authors,
    :attributes,
    keyword_init: true
  )

  # 📐️ An assembly of pieces, connections, layers and groups.
  Design = Struct.new(
    :name,
    :description,
    :icon,
    :image,
    :variant,
    :view,
    :location,
    :unit,
    :typology,
    :created,
    :updated,
    :pieces,
    :connections,
    :authors,
    :attributes,
    keyword_init: true
  )

  # 📦️ The root container for all domain entities.
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
    :typologies,
    :types,
    :designs,
    :attributes,
    keyword_init: true
  )

  # 🏛️ Flattens typology-owned types and designs onto a kit.
  def self.kit_flatten_typologies!(kit)
    kit.types = []
    kit.designs = []
    (kit.typologies || []).each do |topo|
      (topo.types || []).each { |t| kit.types << t }
      (topo.designs || []).each { |d| kit.designs << d }
    end
    kit
  end

  # 🏛️ Packs flat types and designs into a default typology when typologies are absent.
  def self.kit_pack_typologies_from_flat!(kit)
    return kit if kit.typologies && !kit.typologies.empty?

    types = kit.types || []
    designs = kit.designs || []
    return kit if types.empty? && designs.empty?

    topo_id = types.first&.typology || designs.first&.typology || SecureRandom.uuid
    kit.typologies = [
      Typology.new(id_: topo_id, name: 'Default', types: types, designs: designs)
    ]
    kit_flatten_typologies!(kit)
  end
end
