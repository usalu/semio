#pragma once

#using <mscorlib.dll>

using namespace System::Security::Permissions;
[assembly:SecurityPermissionAttribute(SecurityAction::RequestMinimum, SkipVerification=false)];
namespace AbstractionLevels {
    namespace MuehlhausenentwicklungsplanRepository {
        namespace ParameterTypes {
        }
    }
}
namespace AbstractionLevels {
    namespace MuehlhausenentwicklungsplanRepository {
        namespace ObjectTypes {
    ref class Parzelle;
    ref class Strasse;
        }
    }
}
namespace AbstractionLevels {
    namespace MuehlhausenentwicklungsplanRepository {
    ref class Muehlhausenentwicklungsplan;
    }
}
namespace AbstractionLevels {
    namespace StrassenplanRepository {
        namespace ParameterTypes {
        }
    }
}
namespace AbstractionLevels {
    namespace StrassenplanRepository {
        namespace ObjectTypes {
    ref class Gehweg;
        }
    }
}
namespace AbstractionLevels {
    namespace StrassenplanRepository {
    ref class Strassenplan;
    }
}
namespace AbstractionLevels {
    namespace MuehlhausenentwicklungsplanRepository {
        namespace ParameterTypes {
    using namespace System;
    using namespace System;
    
    
    [System::ComponentModel::Description(L"")]
    public enum class Spurzahl
    {
        
        [System::ComponentModel::Description(L"")]
        einspurig = 0,
        
        [System::ComponentModel::Description(L"")]
        zweispurig = 1,
        
        [System::ComponentModel::Description(L"")]
        dreispurig = 2,
    };
        }
    }
}
namespace AbstractionLevels {
    namespace MuehlhausenentwicklungsplanRepository {
        namespace ParameterTypes {
    
        }
    }
}
namespace AbstractionLevels {
    namespace MuehlhausenentwicklungsplanRepository {
        namespace ObjectTypes {
    using namespace System;
    using namespace AbstractionLevels::MuehlhausenentwicklungsplanRepository::ParameterTypes;
    using namespace SemIOLibrary::Parameters;
    using namespace SemIOLibrary::Parameters::GeometryParameters;
    using namespace SemIOLibrary::Parameters::GeometryParameters::Surfaces;
    using namespace SemIOLibrary::Parameters::GeometryParameters::Solids;
    using namespace SemIOLibrary::Parameters::GeometryParameters::Points;
    using namespace SemIOLibrary::Parameters::GeometryParameters::Curves;
    using namespace SemIOLibrary::Parameters::BaseParameters;
    using namespace SemIOLibrary::Parameters::BaseParameters::Numbers;
    using namespace System;
    ref class Parzelle;
    ref class Strasse;
    
    
    [System::ComponentModel::Description(L"")]
    public ref class Parzelle
    {
        
        private: System::Number^  _steigung;
        
        private: Point^  _eingangspunkt;
        
        public: [System::ComponentModel::Description(L"")]
        property System::Number^  Steigung
        {
            System::Number^  get();
            System::Void set(System::Number^  value);
        }
        
        public: [System::ComponentModel::Description(L"")]
        property Point^  Eingangspunkt
        {
            Point^  get();
            System::Void set(Point^  value);
        }
    };
    
    [System::ComponentModel::Description(L"")]
    public ref class Strasse
    {
        
        private: AbstractionLevels::MuehlhausenentwicklungsplanRepository::ParameterTypes::Spurzahl _spur;
        
        public: [System::ComponentModel::Description(L"")]
        property AbstractionLevels::MuehlhausenentwicklungsplanRepository::ParameterTypes::Spurzahl Spur
        {
            AbstractionLevels::MuehlhausenentwicklungsplanRepository::ParameterTypes::Spurzahl get();
            System::Void set(AbstractionLevels::MuehlhausenentwicklungsplanRepository::ParameterTypes::Spurzahl value);
        }
    };
        }
    }
}
namespace AbstractionLevels {
    namespace MuehlhausenentwicklungsplanRepository {
        namespace ObjectTypes {
    
    
    inline System::Number^  Parzelle::Steigung::get()
    {
        return this->_steigung;
    }
    inline System::Void Parzelle::Steigung::set(System::Number^  value)
    {
        this->_steigung = value;
    }
    
    inline Point^  Parzelle::Eingangspunkt::get()
    {
        return this->_eingangspunkt;
    }
    inline System::Void Parzelle::Eingangspunkt::set(Point^  value)
    {
        this->_eingangspunkt = value;
    }
    
    
    inline AbstractionLevels::MuehlhausenentwicklungsplanRepository::ParameterTypes::Spurzahl Strasse::Spur::get()
    {
        return this->_spur;
    }
    inline System::Void Strasse::Spur::set(AbstractionLevels::MuehlhausenentwicklungsplanRepository::ParameterTypes::Spurzahl value)
    {
        this->_spur = value;
    }
        }
    }
}
namespace AbstractionLevels {
    namespace MuehlhausenentwicklungsplanRepository {
    using namespace System;
    using namespace AbstractionLevels::MuehlhausenentwicklungsplanRepository::ObjectTypes;
    using namespace System;
    ref class Muehlhausenentwicklungsplan;
    
    
    [System::ComponentModel::Description(L"")]
    public ref class Muehlhausenentwicklungsplan
    {
        
        private: System::Collections::Generic::List<AbstractionLevels::MuehlhausenentwicklungsplanRepository::ObjectTypes::Parzelle^ >^  _parzelleCollection;
        
        private: System::Collections::Generic::List<AbstractionLevels::MuehlhausenentwicklungsplanRepository::ObjectTypes::Strasse^ >^  _strasseCollection;
        
        public: [System::ComponentModel::Description(L"")]
        property System::Collections::Generic::List<AbstractionLevels::MuehlhausenentwicklungsplanRepository::ObjectTypes::Parzelle^ >^  ParzelleCollection
        {
            System::Collections::Generic::List<AbstractionLevels::MuehlhausenentwicklungsplanRepository::ObjectTypes::Parzelle^ >^  get();
            System::Void set(System::Collections::Generic::List<AbstractionLevels::MuehlhausenentwicklungsplanRepository::ObjectTypes::Parzelle^ >^  value);
        }
        
        public: [System::ComponentModel::Description(L"")]
        property System::Collections::Generic::List<AbstractionLevels::MuehlhausenentwicklungsplanRepository::ObjectTypes::Strasse^ >^  StrasseCollection
        {
            System::Collections::Generic::List<AbstractionLevels::MuehlhausenentwicklungsplanRepository::ObjectTypes::Strasse^ >^  get();
            System::Void set(System::Collections::Generic::List<AbstractionLevels::MuehlhausenentwicklungsplanRepository::ObjectTypes::Strasse^ >^  value);
        }
    };
    }
}
namespace AbstractionLevels {
    namespace MuehlhausenentwicklungsplanRepository {
    
    
    inline System::Collections::Generic::List<AbstractionLevels::MuehlhausenentwicklungsplanRepository::ObjectTypes::Parzelle^ >^  Muehlhausenentwicklungsplan::ParzelleCollection::get()
    {
        return this->_parzelleCollection;
    }
    inline System::Void Muehlhausenentwicklungsplan::ParzelleCollection::set(System::Collections::Generic::List<AbstractionLevels::MuehlhausenentwicklungsplanRepository::ObjectTypes::Parzelle^ >^  value)
    {
    }
    
    inline System::Collections::Generic::List<AbstractionLevels::MuehlhausenentwicklungsplanRepository::ObjectTypes::Strasse^ >^  Muehlhausenentwicklungsplan::StrasseCollection::get()
    {
        return this->_strasseCollection;
    }
    inline System::Void Muehlhausenentwicklungsplan::StrasseCollection::set(System::Collections::Generic::List<AbstractionLevels::MuehlhausenentwicklungsplanRepository::ObjectTypes::Strasse^ >^  value)
    {
    }
    }
}
namespace AbstractionLevels {
    namespace StrassenplanRepository {
        namespace ParameterTypes {
    using namespace System;
    using namespace System;
    
    
    [System::ComponentModel::Description(L"")]
    public enum class Bordsteinkante
    {
        
        [System::ComponentModel::Description(L"")]
        niedrig = 0,
        
        [System::ComponentModel::Description(L"")]
        hoch = 1,
    };
    
    [System::ComponentModel::Description(L""), 
    Flags]
    public enum class Activities
    {
        
        [System::ComponentModel::Description(L"")]
        Durchgangsstrasse = 1,
        
        [System::ComponentModel::Description(L"")]
        Spielstrasse = 2,
    };
        }
    }
}
namespace AbstractionLevels {
    namespace StrassenplanRepository {
        namespace ParameterTypes {
    
    
        }
    }
}
namespace AbstractionLevels {
    namespace StrassenplanRepository {
        namespace ObjectTypes {
    using namespace System;
    using namespace AbstractionLevels::StrassenplanRepository::ParameterTypes;
    using namespace SemIOLibrary::Parameters;
    using namespace SemIOLibrary::Parameters::GeometryParameters;
    using namespace SemIOLibrary::Parameters::GeometryParameters::Surfaces;
    using namespace SemIOLibrary::Parameters::GeometryParameters::Solids;
    using namespace SemIOLibrary::Parameters::GeometryParameters::Points;
    using namespace SemIOLibrary::Parameters::GeometryParameters::Curves;
    using namespace SemIOLibrary::Parameters::BaseParameters;
    using namespace SemIOLibrary::Parameters::BaseParameters::Numbers;
    using namespace System;
    ref class Gehweg;
    
    
    [System::ComponentModel::Description(L"")]
    public ref class Gehweg
    {
        
        private: System::Number^  _breite;
        
        private: AbstractionLevels::StrassenplanRepository::ParameterTypes::Activities _benutzung;
        
        private: AbstractionLevels::StrassenplanRepository::ParameterTypes::Bordsteinkante _hoehe;
        
        public: [System::ComponentModel::Description(L"")]
        property System::Number^  Breite
        {
            System::Number^  get();
            System::Void set(System::Number^  value);
        }
        
        public: [System::ComponentModel::Description(L"")]
        property AbstractionLevels::StrassenplanRepository::ParameterTypes::Activities Benutzung
        {
            AbstractionLevels::StrassenplanRepository::ParameterTypes::Activities get();
            System::Void set(AbstractionLevels::StrassenplanRepository::ParameterTypes::Activities value);
        }
        
        public: [System::ComponentModel::Description(L"")]
        property AbstractionLevels::StrassenplanRepository::ParameterTypes::Bordsteinkante Hoehe
        {
            AbstractionLevels::StrassenplanRepository::ParameterTypes::Bordsteinkante get();
            System::Void set(AbstractionLevels::StrassenplanRepository::ParameterTypes::Bordsteinkante value);
        }
    };
        }
    }
}
namespace AbstractionLevels {
    namespace StrassenplanRepository {
        namespace ObjectTypes {
    
    
    inline System::Number^  Gehweg::Breite::get()
    {
        return this->_breite;
    }
    inline System::Void Gehweg::Breite::set(System::Number^  value)
    {
        this->_breite = value;
    }
    
    inline AbstractionLevels::StrassenplanRepository::ParameterTypes::Activities Gehweg::Benutzung::get()
    {
        return this->_benutzung;
    }
    inline System::Void Gehweg::Benutzung::set(AbstractionLevels::StrassenplanRepository::ParameterTypes::Activities value)
    {
        this->_benutzung = value;
    }
    
    inline AbstractionLevels::StrassenplanRepository::ParameterTypes::Bordsteinkante Gehweg::Hoehe::get()
    {
        return this->_hoehe;
    }
    inline System::Void Gehweg::Hoehe::set(AbstractionLevels::StrassenplanRepository::ParameterTypes::Bordsteinkante value)
    {
        this->_hoehe = value;
    }
        }
    }
}
namespace AbstractionLevels {
    namespace StrassenplanRepository {
    using namespace System;
    using namespace AbstractionLevels::StrassenplanRepository::ObjectTypes;
    using namespace AbstractionLevels::MuehlhausenentwicklungsplanRepository;
    using namespace System;
    ref class Strassenplan;
    
    
    [System::ComponentModel::Description(L"")]
    public ref class Strassenplan : public AbstractionLevels::MuehlhausenentwicklungsplanRepository::Muehlhausenentwicklungsplan
    {
        
        private: System::Collections::Generic::List<AbstractionLevels::StrassenplanRepository::ObjectTypes::Gehweg^ >^  _gehwegCollection;
        
        public: [System::ComponentModel::Description(L"")]
        property System::Collections::Generic::List<AbstractionLevels::StrassenplanRepository::ObjectTypes::Gehweg^ >^  GehwegCollection
        {
            System::Collections::Generic::List<AbstractionLevels::StrassenplanRepository::ObjectTypes::Gehweg^ >^  get();
            System::Void set(System::Collections::Generic::List<AbstractionLevels::StrassenplanRepository::ObjectTypes::Gehweg^ >^  value);
        }
    };
    }
}
namespace AbstractionLevels {
    namespace StrassenplanRepository {
    
    
    inline System::Collections::Generic::List<AbstractionLevels::StrassenplanRepository::ObjectTypes::Gehweg^ >^  Strassenplan::GehwegCollection::get()
    {
        return this->_gehwegCollection;
    }
    inline System::Void Strassenplan::GehwegCollection::set(System::Collections::Generic::List<AbstractionLevels::StrassenplanRepository::ObjectTypes::Gehweg^ >^  value)
    {
    }
    }
}
