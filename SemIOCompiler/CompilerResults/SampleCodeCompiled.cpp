#pragma once

#using <mscorlib.dll>

using namespace System::Security::Permissions;
[assembly:SecurityPermissionAttribute(SecurityAction::RequestMinimum, SkipVerification=false)];
namespace AbstractionLevels {
    namespace MasterplanRepository {
        namespace ParameterTypes {
        }
    }
}
namespace AbstractionLevels {
    namespace MasterplanRepository {
        namespace ThingTypes {
    ref class BuildingVolume;
    ref class BuildingBlock;
        }
    }
}
namespace AbstractionLevels {
    namespace MasterplanRepository {
    ref class Masterplan;
    }
}
namespace AbstractionLevels {
    namespace ApartmentplanRepository {
        namespace ParameterTypes {
        }
    }
}
namespace AbstractionLevels {
    namespace ApartmentplanRepository {
        namespace ThingTypes {
    ref class Apartment;
    ref class Circulation;
        }
    }
}
namespace AbstractionLevels {
    namespace ApartmentplanRepository {
    ref class Apartmentplan;
    }
}
namespace AbstractionLevels {
    namespace RoomplanRepository {
        namespace ParameterTypes {
        }
    }
}
namespace AbstractionLevels {
    namespace RoomplanRepository {
        namespace ThingTypes {
    ref class Opening;
    ref class Window;
    ref class Door;
    ref class Room;
        }
    }
}
namespace AbstractionLevels {
    namespace RoomplanRepository {
    ref class Roomplan;
    }
}
namespace AbstractionLevels {
    namespace MasterplanRepository {
        namespace ParameterTypes {
    using namespace System;
    using namespace System;
    
    
    [System::ComponentModel::Description(L"Defines the designated use of a parcel")]
    public enum class Use
    {
        
        [System::ComponentModel::Description(L"Mainly factories in the landscape")]
        Industrial = 0,
        
        [System::ComponentModel::Description(L"")]
        Residential = 1,
        
        [System::ComponentModel::Description(L"Building areas where buildings are touching and streets are formed by enclosing b" 
L"uildings")]
        Urban = 2,
        
        [System::ComponentModel::Description(L"")]
        Mixed = 3,
    };
    
    [System::ComponentModel::Description(L"Describes an emotion regarding a building"), 
    Flags]
    public enum class Emotions
    {
        
        [System::ComponentModel::Description(L"")]
        Ecstasy = 1,
        
        [System::ComponentModel::Description(L"")]
        Admiration = 2,
        
        [System::ComponentModel::Description(L"")]
        Terror = 4,
        
        [System::ComponentModel::Description(L"")]
        Amazement = 8,
        
        [System::ComponentModel::Description(L"")]
        Grief = 16,
        
        [System::ComponentModel::Description(L"")]
        Loathing = 32,
        
        [System::ComponentModel::Description(L"")]
        Rage = 64,
        
        [System::ComponentModel::Description(L"")]
        Vigilance = 128,
    };
        }
    }
}
namespace AbstractionLevels {
    namespace MasterplanRepository {
        namespace ParameterTypes {
    
    
        }
    }
}
namespace AbstractionLevels {
    namespace MasterplanRepository {
        namespace ThingTypes {
    using namespace System;
    using namespace AbstractionLevels::MasterplanRepository::ParameterTypes;
    using namespace SemIOLibrary::Parameters;
    using namespace SemIOLibrary::Parameters::GeometryParameters;
    using namespace SemIOLibrary::Parameters::GeometryParameters::Surfaces;
    using namespace SemIOLibrary::Parameters::GeometryParameters::Solids;
    using namespace SemIOLibrary::Parameters::GeometryParameters::Points;
    using namespace SemIOLibrary::Parameters::GeometryParameters::Curves;
    using namespace SemIOLibrary::Parameters::BaseParameters;
    using namespace SemIOLibrary::Parameters::BaseParameters::Numbers;
    using namespace System;
    ref class BuildingVolume;
    ref class BuildingBlock;
    
    
    [System::ComponentModel::Description(L"Describes a volume with a use")]
    public ref class BuildingVolume
    {
        
        private: ClosedCurve^  _parcel;
        
        private: AbstractionLevels::MasterplanRepository::ParameterTypes::Use _use;
        
        private: System::Number^  _height;
        
        private: AbstractionLevels::MasterplanRepository::ParameterTypes::Emotions _reactions;
        
        public: [System::ComponentModel::Description(L"")]
        property ClosedCurve^  Parcel
        {
            ClosedCurve^  get();
            System::Void set(ClosedCurve^  value);
        }
        
        public: [System::ComponentModel::Description(L"")]
        property AbstractionLevels::MasterplanRepository::ParameterTypes::Use Use
        {
            AbstractionLevels::MasterplanRepository::ParameterTypes::Use get();
            System::Void set(AbstractionLevels::MasterplanRepository::ParameterTypes::Use value);
        }
        
        public: [System::ComponentModel::Description(L"")]
        property System::Number^  Height
        {
            System::Number^  get();
            System::Void set(System::Number^  value);
        }
        
        public: [System::ComponentModel::Description(L"")]
        property AbstractionLevels::MasterplanRepository::ParameterTypes::Emotions Reactions
        {
            AbstractionLevels::MasterplanRepository::ParameterTypes::Emotions get();
            System::Void set(AbstractionLevels::MasterplanRepository::ParameterTypes::Emotions value);
        }
    };
    
    [System::ComponentModel::Description(L"Describes adjacent building volumes")]
    public ref class BuildingBlock
    {
        
        private: AbstractionLevels::MasterplanRepository::ThingTypes::BuildingVolume^  _buildingVolumes;
        
        public: [System::ComponentModel::Description(L"")]
        property AbstractionLevels::MasterplanRepository::ThingTypes::BuildingVolume^  BuildingVolumes
        {
            AbstractionLevels::MasterplanRepository::ThingTypes::BuildingVolume^  get();
            System::Void set(AbstractionLevels::MasterplanRepository::ThingTypes::BuildingVolume^  value);
        }
    };
        }
    }
}
namespace AbstractionLevels {
    namespace MasterplanRepository {
        namespace ThingTypes {
    
    
    inline ClosedCurve^  BuildingVolume::Parcel::get()
    {
        return this->_parcel;
    }
    inline System::Void BuildingVolume::Parcel::set(ClosedCurve^  value)
    {
        this->_parcel = value;
    }
    
    inline AbstractionLevels::MasterplanRepository::ParameterTypes::Use BuildingVolume::Use::get()
    {
        return this->_use;
    }
    inline System::Void BuildingVolume::Use::set(AbstractionLevels::MasterplanRepository::ParameterTypes::Use value)
    {
        this->_use = value;
    }
    
    inline System::Number^  BuildingVolume::Height::get()
    {
        return this->_height;
    }
    inline System::Void BuildingVolume::Height::set(System::Number^  value)
    {
        this->_height = value;
    }
    
    inline AbstractionLevels::MasterplanRepository::ParameterTypes::Emotions BuildingVolume::Reactions::get()
    {
        return this->_reactions;
    }
    inline System::Void BuildingVolume::Reactions::set(AbstractionLevels::MasterplanRepository::ParameterTypes::Emotions value)
    {
        this->_reactions = value;
    }
    
    
    inline AbstractionLevels::MasterplanRepository::ThingTypes::BuildingVolume^  BuildingBlock::BuildingVolumes::get()
    {
        return this->_buildingVolumes;
    }
    inline System::Void BuildingBlock::BuildingVolumes::set(AbstractionLevels::MasterplanRepository::ThingTypes::BuildingVolume^  value)
    {
        this->_buildingVolumes = value;
    }
        }
    }
}
namespace AbstractionLevels {
    namespace MasterplanRepository {
    using namespace System;
    using namespace AbstractionLevels::MasterplanRepository::ThingTypes;
    using namespace System;
    ref class Masterplan;
    
    
    [System::ComponentModel::Description(L"Massing studies and infrastructure including public transportation (Simillar to 1" 
L"to500)")]
    public ref class Masterplan
    {
        
        private: System::Collections::Generic::List<AbstractionLevels::MasterplanRepository::ThingTypes::BuildingVolume^ >^  _buildingVolumeCollection;
        
        private: System::Collections::Generic::List<AbstractionLevels::MasterplanRepository::ThingTypes::BuildingBlock^ >^  _buildingBlockCollection;
        
        public: [System::ComponentModel::Description(L"Describes a volume with a use")]
        property System::Collections::Generic::List<AbstractionLevels::MasterplanRepository::ThingTypes::BuildingVolume^ >^  BuildingVolumeCollection
        {
            System::Collections::Generic::List<AbstractionLevels::MasterplanRepository::ThingTypes::BuildingVolume^ >^  get();
            System::Void set(System::Collections::Generic::List<AbstractionLevels::MasterplanRepository::ThingTypes::BuildingVolume^ >^  value);
        }
        
        public: [System::ComponentModel::Description(L"Describes adjacent building volumes")]
        property System::Collections::Generic::List<AbstractionLevels::MasterplanRepository::ThingTypes::BuildingBlock^ >^  BuildingBlockCollection
        {
            System::Collections::Generic::List<AbstractionLevels::MasterplanRepository::ThingTypes::BuildingBlock^ >^  get();
            System::Void set(System::Collections::Generic::List<AbstractionLevels::MasterplanRepository::ThingTypes::BuildingBlock^ >^  value);
        }
    };
    }
}
namespace AbstractionLevels {
    namespace MasterplanRepository {
    
    
    inline System::Collections::Generic::List<AbstractionLevels::MasterplanRepository::ThingTypes::BuildingVolume^ >^  Masterplan::BuildingVolumeCollection::get()
    {
        return this->_buildingVolumeCollection;
    }
    inline System::Void Masterplan::BuildingVolumeCollection::set(System::Collections::Generic::List<AbstractionLevels::MasterplanRepository::ThingTypes::BuildingVolume^ >^  value)
    {
    }
    
    inline System::Collections::Generic::List<AbstractionLevels::MasterplanRepository::ThingTypes::BuildingBlock^ >^  Masterplan::BuildingBlockCollection::get()
    {
        return this->_buildingBlockCollection;
    }
    inline System::Void Masterplan::BuildingBlockCollection::set(System::Collections::Generic::List<AbstractionLevels::MasterplanRepository::ThingTypes::BuildingBlock^ >^  value)
    {
    }
    }
}
namespace AbstractionLevels {
    namespace ApartmentplanRepository {
        namespace ParameterTypes {
    using namespace System;
    using namespace System;
    
    
    [System::ComponentModel::Description(L"Describing how an aprartment can be reached with")]
    public enum class Accessability
    {
        
        [System::ComponentModel::Description(L"")]
        Walking = 0,
        
        [System::ComponentModel::Description(L"")]
        Rollator = 1,
        
        [System::ComponentModel::Description(L"")]
        Wheelchair = 2,
    };
        }
    }
}
namespace AbstractionLevels {
    namespace ApartmentplanRepository {
        namespace ParameterTypes {
    
        }
    }
}
namespace AbstractionLevels {
    namespace ApartmentplanRepository {
        namespace ThingTypes {
    using namespace System;
    using namespace AbstractionLevels::ApartmentplanRepository::ParameterTypes;
    using namespace SemIOLibrary::Parameters;
    using namespace SemIOLibrary::Parameters::GeometryParameters;
    using namespace SemIOLibrary::Parameters::GeometryParameters::Surfaces;
    using namespace SemIOLibrary::Parameters::GeometryParameters::Solids;
    using namespace SemIOLibrary::Parameters::GeometryParameters::Points;
    using namespace SemIOLibrary::Parameters::GeometryParameters::Curves;
    using namespace SemIOLibrary::Parameters::BaseParameters;
    using namespace SemIOLibrary::Parameters::BaseParameters::Numbers;
    using namespace System;
    ref class Apartment;
    ref class Circulation;
    
    
    [System::ComponentModel::Description(L"Most general information about an apartment")]
    public ref class Apartment
    {
        
        private: ClosedCurve^  _boundary;
        
        private: Point^  _entrance;
        
        private: Integer^  _countPeople;
        
        private: AbstractionLevels::ApartmentplanRepository::ParameterTypes::Accessability _accessability;
        
        public: [System::ComponentModel::Description(L"Only one storey footpring of apartment.")]
        property ClosedCurve^  Boundary
        {
            ClosedCurve^  get();
            System::Void set(ClosedCurve^  value);
        }
        
        public: [System::ComponentModel::Description(L"")]
        property Point^  Entrance
        {
            Point^  get();
            System::Void set(Point^  value);
        }
        
        public: [System::ComponentModel::Description(L"")]
        property Integer^  CountPeople
        {
            Integer^  get();
            System::Void set(Integer^  value);
        }
        
        public: [System::ComponentModel::Description(L"")]
        property AbstractionLevels::ApartmentplanRepository::ParameterTypes::Accessability Accessability
        {
            AbstractionLevels::ApartmentplanRepository::ParameterTypes::Accessability get();
            System::Void set(AbstractionLevels::ApartmentplanRepository::ParameterTypes::Accessability value);
        }
    };
    
    [System::ComponentModel::Description(L"General inforamtion about how the building is accessed")]
    public ref class Circulation
    {
        
        private: ClosedCurve^  _boundary;
        
        private: Point^  _mainEntrances;
        
        private: ClosedCurve^  _staircase;
        
        public: [System::ComponentModel::Description(L"")]
        property ClosedCurve^  Boundary
        {
            ClosedCurve^  get();
            System::Void set(ClosedCurve^  value);
        }
        
        public: [System::ComponentModel::Description(L"")]
        property Point^  MainEntrances
        {
            Point^  get();
            System::Void set(Point^  value);
        }
        
        public: [System::ComponentModel::Description(L"")]
        property ClosedCurve^  Staircase
        {
            ClosedCurve^  get();
            System::Void set(ClosedCurve^  value);
        }
    };
        }
    }
}
namespace AbstractionLevels {
    namespace ApartmentplanRepository {
        namespace ThingTypes {
    
    
    inline ClosedCurve^  Apartment::Boundary::get()
    {
        return this->_boundary;
    }
    inline System::Void Apartment::Boundary::set(ClosedCurve^  value)
    {
        this->_boundary = value;
    }
    
    inline Point^  Apartment::Entrance::get()
    {
        return this->_entrance;
    }
    inline System::Void Apartment::Entrance::set(Point^  value)
    {
        this->_entrance = value;
    }
    
    inline Integer^  Apartment::CountPeople::get()
    {
        return this->_countPeople;
    }
    inline System::Void Apartment::CountPeople::set(Integer^  value)
    {
        this->_countPeople = value;
    }
    
    inline AbstractionLevels::ApartmentplanRepository::ParameterTypes::Accessability Apartment::Accessability::get()
    {
        return this->_accessability;
    }
    inline System::Void Apartment::Accessability::set(AbstractionLevels::ApartmentplanRepository::ParameterTypes::Accessability value)
    {
        this->_accessability = value;
    }
    
    
    inline ClosedCurve^  Circulation::Boundary::get()
    {
        return this->_boundary;
    }
    inline System::Void Circulation::Boundary::set(ClosedCurve^  value)
    {
        this->_boundary = value;
    }
    
    inline Point^  Circulation::MainEntrances::get()
    {
        return this->_mainEntrances;
    }
    inline System::Void Circulation::MainEntrances::set(Point^  value)
    {
        this->_mainEntrances = value;
    }
    
    inline ClosedCurve^  Circulation::Staircase::get()
    {
        return this->_staircase;
    }
    inline System::Void Circulation::Staircase::set(ClosedCurve^  value)
    {
        this->_staircase = value;
    }
        }
    }
}
namespace AbstractionLevels {
    namespace ApartmentplanRepository {
    using namespace System;
    using namespace AbstractionLevels::ApartmentplanRepository::ThingTypes;
    using namespace AbstractionLevels::MasterplanRepository;
    using namespace System;
    ref class Apartmentplan;
    
    
    [System::ComponentModel::Description(L"General information about layout and accessability of the apartment equivalent to" 
L" 1to200 in analogue scale")]
    public ref class Apartmentplan : public AbstractionLevels::MasterplanRepository::Masterplan
    {
        
        private: System::Collections::Generic::List<AbstractionLevels::ApartmentplanRepository::ThingTypes::Apartment^ >^  _apartmentCollection;
        
        private: System::Collections::Generic::List<AbstractionLevels::ApartmentplanRepository::ThingTypes::Circulation^ >^  _circulationCollection;
        
        public: [System::ComponentModel::Description(L"Most general information about an apartment")]
        property System::Collections::Generic::List<AbstractionLevels::ApartmentplanRepository::ThingTypes::Apartment^ >^  ApartmentCollection
        {
            System::Collections::Generic::List<AbstractionLevels::ApartmentplanRepository::ThingTypes::Apartment^ >^  get();
            System::Void set(System::Collections::Generic::List<AbstractionLevels::ApartmentplanRepository::ThingTypes::Apartment^ >^  value);
        }
        
        public: [System::ComponentModel::Description(L"General inforamtion about how the building is accessed")]
        property System::Collections::Generic::List<AbstractionLevels::ApartmentplanRepository::ThingTypes::Circulation^ >^  CirculationCollection
        {
            System::Collections::Generic::List<AbstractionLevels::ApartmentplanRepository::ThingTypes::Circulation^ >^  get();
            System::Void set(System::Collections::Generic::List<AbstractionLevels::ApartmentplanRepository::ThingTypes::Circulation^ >^  value);
        }
    };
    }
}
namespace AbstractionLevels {
    namespace ApartmentplanRepository {
    
    
    inline System::Collections::Generic::List<AbstractionLevels::ApartmentplanRepository::ThingTypes::Apartment^ >^  Apartmentplan::ApartmentCollection::get()
    {
        return this->_apartmentCollection;
    }
    inline System::Void Apartmentplan::ApartmentCollection::set(System::Collections::Generic::List<AbstractionLevels::ApartmentplanRepository::ThingTypes::Apartment^ >^  value)
    {
    }
    
    inline System::Collections::Generic::List<AbstractionLevels::ApartmentplanRepository::ThingTypes::Circulation^ >^  Apartmentplan::CirculationCollection::get()
    {
        return this->_circulationCollection;
    }
    inline System::Void Apartmentplan::CirculationCollection::set(System::Collections::Generic::List<AbstractionLevels::ApartmentplanRepository::ThingTypes::Circulation^ >^  value)
    {
    }
    }
}
namespace AbstractionLevels {
    namespace RoomplanRepository {
        namespace ParameterTypes {
    using namespace System;
    using namespace System;
    
    
    [System::ComponentModel::Description(L"Direction of door wing")]
    public enum class DoorDirection
    {
        
        [System::ComponentModel::Description(L"")]
        Inside = 0,
        
        [System::ComponentModel::Description(L"")]
        Outside = 1,
    };
    
    [System::ComponentModel::Description(L"Activities that happen in a room"), 
    Flags]
    public enum class Activities
    {
        
        [System::ComponentModel::Description(L"")]
        Sleeping = 1,
        
        [System::ComponentModel::Description(L"")]
        Relaxing = 2,
        
        [System::ComponentModel::Description(L"")]
        Working = 4,
        
        [System::ComponentModel::Description(L"")]
        Cooking = 8,
        
        [System::ComponentModel::Description(L"")]
        Showering = 16,
        
        [System::ComponentModel::Description(L"")]
        Bathing = 32,
    };
    
    [System::ComponentModel::Description(L"Describing how much noise will be emmited from the room")]
    public enum class NoiseLevel
    {
        
        [System::ComponentModel::Description(L"under 20 dB")]
        VeryQuiet = 0,
        
        [System::ComponentModel::Description(L"20 - 40 dB")]
        Quiet = 1,
        
        [System::ComponentModel::Description(L"40 - 60 dB")]
        ConversationLevel = 2,
        
        [System::ComponentModel::Description(L"over 60 dB")]
        Loud = 3,
    };
        }
    }
}
namespace AbstractionLevels {
    namespace RoomplanRepository {
        namespace ParameterTypes {
    
    
    
        }
    }
}
namespace AbstractionLevels {
    namespace RoomplanRepository {
        namespace ThingTypes {
    using namespace System;
    using namespace AbstractionLevels::RoomplanRepository::ParameterTypes;
    using namespace SemIOLibrary::Parameters;
    using namespace SemIOLibrary::Parameters::GeometryParameters;
    using namespace SemIOLibrary::Parameters::GeometryParameters::Surfaces;
    using namespace SemIOLibrary::Parameters::GeometryParameters::Solids;
    using namespace SemIOLibrary::Parameters::GeometryParameters::Points;
    using namespace SemIOLibrary::Parameters::GeometryParameters::Curves;
    using namespace SemIOLibrary::Parameters::BaseParameters;
    using namespace SemIOLibrary::Parameters::BaseParameters::Numbers;
    using namespace System;
    ref class Opening;
    ref class Window;
    ref class Door;
    ref class Room;
    
    
    [System::ComponentModel::Description(L"Generic base class for openings")]
    public ref class Opening
    {
        
        private: Point^  _center;
        
        private: System::Number^  _width;
        
        private: System::Number^  _height;
        
        public: [System::ComponentModel::Description(L"")]
        property Point^  Center
        {
            Point^  get();
            System::Void set(Point^  value);
        }
        
        public: [System::ComponentModel::Description(L"")]
        property System::Number^  Width
        {
            System::Number^  get();
            System::Void set(System::Number^  value);
        }
        
        public: [System::ComponentModel::Description(L"")]
        property System::Number^  Height
        {
            System::Number^  get();
            System::Void set(System::Number^  value);
        }
    };
    
    [System::ComponentModel::Description(L"Abstract window definition")]
    public ref class Window : public AbstractionLevels::RoomplanRepository::ThingTypes::Opening
    {
        
        private: System::Number^  _parapetHeight;
        
        public: [System::ComponentModel::Description(L"")]
        property System::Number^  ParapetHeight
        {
            System::Number^  get();
            System::Void set(System::Number^  value);
        }
    };
    
    [System::ComponentModel::Description(L"Abstract door definition")]
    public ref class Door : public AbstractionLevels::RoomplanRepository::ThingTypes::Opening
    {
        
        private: AbstractionLevels::RoomplanRepository::ParameterTypes::DoorDirection _doorDirection;
        
        public: [System::ComponentModel::Description(L"")]
        property AbstractionLevels::RoomplanRepository::ParameterTypes::DoorDirection DoorDirection
        {
            AbstractionLevels::RoomplanRepository::ParameterTypes::DoorDirection get();
            System::Void set(AbstractionLevels::RoomplanRepository::ParameterTypes::DoorDirection value);
        }
    };
    
    [System::ComponentModel::Description(L"Abstract definition of a room with doors and windows")]
    public ref class Room
    {
        
        private: ClosedCurve^  _boundary;
        
        private: AbstractionLevels::RoomplanRepository::ThingTypes::Window^  _windows;
        
        private: AbstractionLevels::RoomplanRepository::ThingTypes::Door^  _doors;
        
        private: AbstractionLevels::RoomplanRepository::ParameterTypes::Activities _activities;
        
        public: [System::ComponentModel::Description(L"")]
        property ClosedCurve^  Boundary
        {
            ClosedCurve^  get();
            System::Void set(ClosedCurve^  value);
        }
        
        public: [System::ComponentModel::Description(L"")]
        property AbstractionLevels::RoomplanRepository::ThingTypes::Window^  Windows
        {
            AbstractionLevels::RoomplanRepository::ThingTypes::Window^  get();
            System::Void set(AbstractionLevels::RoomplanRepository::ThingTypes::Window^  value);
        }
        
        public: [System::ComponentModel::Description(L"")]
        property AbstractionLevels::RoomplanRepository::ThingTypes::Door^  Doors
        {
            AbstractionLevels::RoomplanRepository::ThingTypes::Door^  get();
            System::Void set(AbstractionLevels::RoomplanRepository::ThingTypes::Door^  value);
        }
        
        public: [System::ComponentModel::Description(L"")]
        property AbstractionLevels::RoomplanRepository::ParameterTypes::Activities Activities
        {
            AbstractionLevels::RoomplanRepository::ParameterTypes::Activities get();
            System::Void set(AbstractionLevels::RoomplanRepository::ParameterTypes::Activities value);
        }
    };
        }
    }
}
namespace AbstractionLevels {
    namespace RoomplanRepository {
        namespace ThingTypes {
    
    
    inline Point^  Opening::Center::get()
    {
        return this->_center;
    }
    inline System::Void Opening::Center::set(Point^  value)
    {
        this->_center = value;
    }
    
    inline System::Number^  Opening::Width::get()
    {
        return this->_width;
    }
    inline System::Void Opening::Width::set(System::Number^  value)
    {
        this->_width = value;
    }
    
    inline System::Number^  Opening::Height::get()
    {
        return this->_height;
    }
    inline System::Void Opening::Height::set(System::Number^  value)
    {
        this->_height = value;
    }
    
    
    inline System::Number^  Window::ParapetHeight::get()
    {
        return this->_parapetHeight;
    }
    inline System::Void Window::ParapetHeight::set(System::Number^  value)
    {
        this->_parapetHeight = value;
    }
    
    
    inline AbstractionLevels::RoomplanRepository::ParameterTypes::DoorDirection Door::DoorDirection::get()
    {
        return this->_doorDirection;
    }
    inline System::Void Door::DoorDirection::set(AbstractionLevels::RoomplanRepository::ParameterTypes::DoorDirection value)
    {
        this->_doorDirection = value;
    }
    
    
    inline ClosedCurve^  Room::Boundary::get()
    {
        return this->_boundary;
    }
    inline System::Void Room::Boundary::set(ClosedCurve^  value)
    {
        this->_boundary = value;
    }
    
    inline AbstractionLevels::RoomplanRepository::ThingTypes::Window^  Room::Windows::get()
    {
        return this->_windows;
    }
    inline System::Void Room::Windows::set(AbstractionLevels::RoomplanRepository::ThingTypes::Window^  value)
    {
        this->_windows = value;
    }
    
    inline AbstractionLevels::RoomplanRepository::ThingTypes::Door^  Room::Doors::get()
    {
        return this->_doors;
    }
    inline System::Void Room::Doors::set(AbstractionLevels::RoomplanRepository::ThingTypes::Door^  value)
    {
        this->_doors = value;
    }
    
    inline AbstractionLevels::RoomplanRepository::ParameterTypes::Activities Room::Activities::get()
    {
        return this->_activities;
    }
    inline System::Void Room::Activities::set(AbstractionLevels::RoomplanRepository::ParameterTypes::Activities value)
    {
        this->_activities = value;
    }
        }
    }
}
namespace AbstractionLevels {
    namespace RoomplanRepository {
    using namespace System;
    using namespace AbstractionLevels::RoomplanRepository::ThingTypes;
    using namespace AbstractionLevels::ApartmentplanRepository;
    using namespace System;
    ref class Roomplan;
    
    
    [System::ComponentModel::Description(L"Inforamtion about rooms, relations between them and openings(windows and doors). " 
L"Comparable to 1to100")]
    public ref class Roomplan : public AbstractionLevels::ApartmentplanRepository::Apartmentplan
    {
        
        private: System::Collections::Generic::List<AbstractionLevels::RoomplanRepository::ThingTypes::Opening^ >^  _openingCollection;
        
        private: System::Collections::Generic::List<AbstractionLevels::RoomplanRepository::ThingTypes::Window^ >^  _windowCollection;
        
        private: System::Collections::Generic::List<AbstractionLevels::RoomplanRepository::ThingTypes::Door^ >^  _doorCollection;
        
        private: System::Collections::Generic::List<AbstractionLevels::RoomplanRepository::ThingTypes::Room^ >^  _roomCollection;
        
        public: [System::ComponentModel::Description(L"Generic base class for openings")]
        property System::Collections::Generic::List<AbstractionLevels::RoomplanRepository::ThingTypes::Opening^ >^  OpeningCollection
        {
            System::Collections::Generic::List<AbstractionLevels::RoomplanRepository::ThingTypes::Opening^ >^  get();
            System::Void set(System::Collections::Generic::List<AbstractionLevels::RoomplanRepository::ThingTypes::Opening^ >^  value);
        }
        
        public: [System::ComponentModel::Description(L"Abstract window definition")]
        property System::Collections::Generic::List<AbstractionLevels::RoomplanRepository::ThingTypes::Window^ >^  WindowCollection
        {
            System::Collections::Generic::List<AbstractionLevels::RoomplanRepository::ThingTypes::Window^ >^  get();
            System::Void set(System::Collections::Generic::List<AbstractionLevels::RoomplanRepository::ThingTypes::Window^ >^  value);
        }
        
        public: [System::ComponentModel::Description(L"Abstract door definition")]
        property System::Collections::Generic::List<AbstractionLevels::RoomplanRepository::ThingTypes::Door^ >^  DoorCollection
        {
            System::Collections::Generic::List<AbstractionLevels::RoomplanRepository::ThingTypes::Door^ >^  get();
            System::Void set(System::Collections::Generic::List<AbstractionLevels::RoomplanRepository::ThingTypes::Door^ >^  value);
        }
        
        public: [System::ComponentModel::Description(L"Abstract definition of a room with doors and windows")]
        property System::Collections::Generic::List<AbstractionLevels::RoomplanRepository::ThingTypes::Room^ >^  RoomCollection
        {
            System::Collections::Generic::List<AbstractionLevels::RoomplanRepository::ThingTypes::Room^ >^  get();
            System::Void set(System::Collections::Generic::List<AbstractionLevels::RoomplanRepository::ThingTypes::Room^ >^  value);
        }
    };
    }
}
namespace AbstractionLevels {
    namespace RoomplanRepository {
    
    
    inline System::Collections::Generic::List<AbstractionLevels::RoomplanRepository::ThingTypes::Opening^ >^  Roomplan::OpeningCollection::get()
    {
        return this->_openingCollection;
    }
    inline System::Void Roomplan::OpeningCollection::set(System::Collections::Generic::List<AbstractionLevels::RoomplanRepository::ThingTypes::Opening^ >^  value)
    {
    }
    
    inline System::Collections::Generic::List<AbstractionLevels::RoomplanRepository::ThingTypes::Window^ >^  Roomplan::WindowCollection::get()
    {
        return this->_windowCollection;
    }
    inline System::Void Roomplan::WindowCollection::set(System::Collections::Generic::List<AbstractionLevels::RoomplanRepository::ThingTypes::Window^ >^  value)
    {
    }
    
    inline System::Collections::Generic::List<AbstractionLevels::RoomplanRepository::ThingTypes::Door^ >^  Roomplan::DoorCollection::get()
    {
        return this->_doorCollection;
    }
    inline System::Void Roomplan::DoorCollection::set(System::Collections::Generic::List<AbstractionLevels::RoomplanRepository::ThingTypes::Door^ >^  value)
    {
    }
    
    inline System::Collections::Generic::List<AbstractionLevels::RoomplanRepository::ThingTypes::Room^ >^  Roomplan::RoomCollection::get()
    {
        return this->_roomCollection;
    }
    inline System::Void Roomplan::RoomCollection::set(System::Collections::Generic::List<AbstractionLevels::RoomplanRepository::ThingTypes::Room^ >^  value)
    {
    }
    }
}
