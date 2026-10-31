# Independent Discipline Membership Audit R95

No source or fixture edits were made. The source hash inspected is 029dca650a9e32e24b06ff5d339f6af6dc970c3a1c9658a3e3c63f16b1e13039.

The explicit source model.element_tags controls partial_model; metadata.domains does not automatically generate membership. The immutable snapshot fixture explicitly preserves source-only membership. Both labels and explicit membership should nevertheless be intentionally consistent; refreshing expectations alone would conceal stale product identity.

## Recommended Product Decisions

- Remove FIX-PARENTS-STORAGE from services. It is an IfcFurnishingElement storage cabinet with window_storage shell/shelf/top roles and inferred sliding closure. Its current architecture-only metadata is correct. Shared parents-suite historical text mentioning a service plinth does not turn this independent storage product into a sanitary terminal.
- Retain FIX-LAUNDRY in services, add it to architecture, and add services to metadata.domains. It is explicitly IfcSanitaryTerminal/SINK with hollow trough_floor/wall/rear_deck geometry, sourced laundry function and drains. It should match SINK-KITCHEN-1/2 and SINK-PANTRY discipline sharing. Keep its qualified historical uncertainty and geometry unchanged.
- Add architecture to metadata.domains for HEAT-RISER-6-RETURN and HEAT-RISER-11-RETURN. Both already belong explicitly to architecture and services; they are visible paired return pipes. Keep their services engineering author.
- Remove services from FIX-DRESSING-STORAGE metadata. Its concrete cabinet frame/shelf, inferred painted closure/handle and recessed kick are furnishing construction; it has no authored sanitary role or service system. It is already absent from the services roster.
- Add the nine first-floor WC partition/opening/door products listed below to services. They already carry architecture+services metadata; the explicit ground-floor WC roster consistently includes the corresponding sanitary cubicle construction. This retains the existing coordination scope rather than stripping services labels from only the first-floor equivalents.
- Add all seven omitted architecture-domain products listed below to architecture. They include the published soffit/gravel finishes, central handrail and roof beds; this restores actual architectural output coverage.

## Exact Roster Additions

Architecture:

- FIX-LAUNDRY
- FIN-SOFFIT-RF-2A
- FIN-SOFFIT-RF-2B
- FIN-PILOTIS-GRAVEL
- TUBE-STAIR-CENTRAL
- FIX-ROOF-BED-1
- FIX-ROOF-BED-2

Services:

- W-WC-FIRST-N
- W-WC-FIRST-S
- W-WC-FIRST-W
- W-WC-FIRST-E
- W-WC-FIRST-C
- OP-D-WC-FIRST-WEST
- D-WC-FIRST-WEST
- OP-D-WC-FIRST-EAST
- D-WC-FIRST-EAST

## Portable Core Fixture Refresh

After the above source corrections, use the following manually reviewed service class roster and shared tag sets. The current fixture's eight-class restriction and short shared list predate sanitary, lighting, appliance and furniture work. The single shared_services HOIST-JIB remains valid.

- IfcBoiler
- IfcBuildingElementProxy
- IfcChimney
- IfcCovering
- IfcDoor
- IfcFurnishingElement
- IfcLightFixture
- IfcOpeningElement
- IfcPipeSegment
- IfcSanitaryTerminal
- IfcSpaceHeater
- IfcWall
- IfcWasteTerminal

### architecture

- BASIN-PARENTS
- BATH-CHAUFFEUR
- BIDET-CHAUFFEUR
- BIDET-GUEST
- BIDET-PARENTS
- BIDET-SHARED
- CHIMNEY-BOUDOIR-LOW
- CHIMNEY-KITCHEN-ROOF
- CHIMNEY-LINGERIE-LOW
- CHIMNEY-SALON-ROOF
- COLLECTOR-KITCHEN-PATIO-9
- D-WC-CHAUFFEUR
- D-WC-FIRST-EAST
- D-WC-FIRST-WEST
- D-WC-GROUND-SERVICE
- FIX-BATH
- FIX-CHAISE
- FIX-CHAUFFEUR-BASIN
- FIX-GUEST-BASIN
- FIX-HALL-BASIN
- FIX-LAUNDRY
- FIX-PARENTS-WET-PLATFORM
- FIX-SERVANT-BASIN-1
- FIX-SERVANT-BASIN-2
- FIX-SERVANT-RADIATOR-1
- FIX-SERVANT-RADIATOR-2
- FIX-SHARED-BASIN
- FIX-SHARED-BATH
- HEAT-RISER-11-RETURN
- HEAT-RISER-6-RETURN
- HEAT-RISER-VESTIBULE-5-MOUNTS
- HEAT-RISER-VESTIBULE-5-RETURN
- HEAT-RISER-VESTIBULE-5-SUPPLY
- HOIST-JIB
- KITCHEN-APPLIANCE-CYLINDER
- KITCHEN-APPLIANCE-PLATE
- KITCHEN-APPLIANCE-STAND
- LIGHT-EXTERIOR-11
- LIGHT-EXTERIOR-6
- LIGHT-EXTERIOR-REAR-E
- LIGHT-EXTERIOR-REAR-W
- LIGHT-FIRST-HALL-CEILING
- LIGHT-HALL-BASIN-19
- LIGHT-HALL-CEILING
- LIGHT-HALL-ZEISS-1
- LIGHT-HALL-ZEISS-2
- LIGHT-KITCHEN-1
- LIGHT-KITCHEN-2
- LIGHT-SALON-LINEAR
- OP-BOUDOIR-LOW-ROOF
- OP-D-WC-CHAUFFEUR
- OP-D-WC-FIRST-EAST
- OP-D-WC-FIRST-WEST
- OP-D-WC-GROUND-SERVICE
- OP-DP-1-FLOOR
- OP-DP-1-ROOF
- OP-DP-2-FLOOR
- OP-DP-2-ROOF
- OP-FLUE-FLOOR
- OP-FLUE-ROOF
- OP-HEAT-5-SL1
- OP-KITCHEN-FLUE-ROOF
- OP-LINGERIE-LOW-ROOF
- OP-RAIN-9-SL1
- OP-SALON-FLUE-ROOF
- OP-SOIL-FLOOR
- PIPE-KITCHEN-VISIBLE-1
- PIPE-KITCHEN-VISIBLE-2
- PIPE-KITCHEN-VISIBLE-3
- RAD-1
- RAD-HALL-BEHIND-STAIR
- RAD-HALL-GARAGE
- RAD-KITCHEN-SINK
- RAIN-KITCHEN-PATIO-9
- RAIN-KITCHEN-PATIO-9-COVER
- SINK-KITCHEN-1
- SINK-KITCHEN-2
- SINK-PANTRY
- TAP-GUEST
- TAP-HALL
- TAP-KITCHEN
- TAP-KITCHEN-2
- TAP-PANTRY
- TILE-SERVANT-BASIN-1
- TILE-SERVANT-BASIN-2
- W-WC-CHAUFFEUR-CURVE
- W-WC-FIRST-C
- W-WC-FIRST-E
- W-WC-FIRST-N
- W-WC-FIRST-S
- W-WC-FIRST-W
- W-WC-GROUND-SERVICE-E
- W-WC-GROUND-SERVICE-N
- WC-CHAUFFEUR
- WC-FIRST-EAST
- WC-FIRST-WEST
- WC-GROUND-SERVICE

### structure

- HOIST-JIB


These are exact proposed expectations, not a passing test claim. Root should rerun the six source-only contracts and the full suite against the final source. Do not replace explicit source model.element_tags with class-based or metadata-based inference.
