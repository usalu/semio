#pragma once

#using <mscorlib.dll>

using namespace System::Security::Permissions;
[assembly:SecurityPermissionAttribute(SecurityAction::RequestMinimum, SkipVerification=false)];
namespace AbstractionLevels {
    namespace ABSTRACTIONLEVELNAMERepository {
        namespace ParameterTypes {
        }
    }
}
namespace AbstractionLevels {
    namespace ABSTRACTIONLEVELNAMERepository {
        namespace ObjectTypes {
    ref class OBJECTNAME;
    ref class ANOTHEROBJECTNAME;
        }
    }
}
namespace AbstractionLevels {
    namespace ABSTRACTIONLEVELNAMERepository {
    ref class ABSTRACTIONLEVELNAME;
    }
}
namespace AbstractionLevels {
    namespace ANOTHERABSTRACTIONLEVELNAMERepository {
        namespace ParameterTypes {
        }
    }
}
namespace AbstractionLevels {
    namespace ANOTHERABSTRACTIONLEVELNAMERepository {
        namespace ObjectTypes {
    ref class OBJECTNAME;
        }
    }
}
namespace AbstractionLevels {
    namespace ANOTHERABSTRACTIONLEVELNAMERepository {
    ref class ANOTHERABSTRACTIONLEVELNAME;
    }
}
namespace AbstractionLevels {
    namespace ABSTRACTIONLEVELNAMERepository {
        namespace ParameterTypes {
    using namespace System;
    using namespace System;
    
    
    [System::ComponentModel::Description(L"Your description for the PARAMETERTYPENAME. You can use the PARAMETERTYPENAME as " 
L"PARAMETERTYPE for an Object")]
    public enum class PARAMETERTYPENAME
    {
        
        [System::ComponentModel::Description(L"Your description for the discrete VALUE that PARAMETERTYPENAME can take.")]
        VALUE = 0,
        
        [System::ComponentModel::Description(L"Your description for the discrete ANOTHERVALUE that PARAMETERTYPENAME can take. T" 
L"his doesn\'t really need \r\na descrption so I will leave the next description away" 
L".")]
        ANOTHERVALUE = 1,
        
        [System::ComponentModel::Description(L"")]
        ONEMOREVALUE = 2,
        
        [System::ComponentModel::Description(L"Your description for the discrete LASTVALUE. Of course feel free to add as many m" 
L"ore values as you wish")]
        LASTVALUE = 3,
    };
    
    [System::ComponentModel::Description(L"Note that ANOTHERPARAMETERTYPENAME can take multiple values at the same time beca" 
L"use of the [Multiple] tag."), 
    Flags]
    public enum class ANOTHERPARAMETERTYPENAME
    {
        
        [System::ComponentModel::Description(L"")]
        VALUE = 1,
        
        [System::ComponentModel::Description(L"")]
        SECONDVALUE = 2,
        
        [System::ComponentModel::Description(L"")]
        THATSENOUGHVALUE = 4,
    };
        }
    }
}
namespace AbstractionLevels {
    namespace ABSTRACTIONLEVELNAMERepository {
        namespace ParameterTypes {
    
    
        }
    }
}
namespace AbstractionLevels {
    namespace ABSTRACTIONLEVELNAMERepository {
        namespace ObjectTypes {
    using namespace System;
    using namespace AbstractionLevels::ABSTRACTIONLEVELNAMERepository::ParameterTypes;
    using namespace SemIOLibrary::Parameters;
    using namespace SemIOLibrary::Parameters::GeometryParameters;
    using namespace SemIOLibrary::Parameters::GeometryParameters::Surfaces;
    using namespace SemIOLibrary::Parameters::GeometryParameters::Solids;
    using namespace SemIOLibrary::Parameters::GeometryParameters::Points;
    using namespace SemIOLibrary::Parameters::GeometryParameters::Curves;
    using namespace SemIOLibrary::Parameters::BaseParameters;
    using namespace SemIOLibrary::Parameters::BaseParameters::Numbers;
    using namespace System;
    ref class OBJECTNAME;
    ref class ANOTHEROBJECTNAME;
    
    
    [System::ComponentModel::Description(L"This is your first Object that has parameters. The PARAMETERTYPE can be a Paramet" 
L"er from above \r\n\tor a standard one like Point, Text, Curve, Number, ...")]
    public ref class OBJECTNAME
    {
        
        private: Text^  _pARMATERNAMEOFTEXT;
        
        private: Point^  _pARMATERNAMEOFPoint;
        
        private: AbstractionLevels::ABSTRACTIONLEVELNAMERepository::ParameterTypes::PARAMETERTYPENAME _pARAMTERTYPENAMENAME;
        
        private: AbstractionLevels::ABSTRACTIONLEVELNAMERepository::ParameterTypes::ANOTHERPARAMETERTYPENAME _aNOTHERPARAMETERTYPENAME;
        
        public: [System::ComponentModel::Description(L"Does it need to start with text\? No of course not. Feel free to add any parameter" 
L" to define an object")]
        property Text^  PARMATERNAMEOFTEXT
        {
            Text^  get();
            System::Void set(Text^  value);
        }
        
        public: [System::ComponentModel::Description(L"")]
        property Point^  PARMATERNAMEOFPoint
        {
            Point^  get();
            System::Void set(Point^  value);
        }
        
        public: [System::ComponentModel::Description(L"Wowoo this looks funky. Actually it is only a Paramter from above with a name.")]
        property AbstractionLevels::ABSTRACTIONLEVELNAMERepository::ParameterTypes::PARAMETERTYPENAME PARAMTERTYPENAMENAME
        {
            AbstractionLevels::ABSTRACTIONLEVELNAMERepository::ParameterTypes::PARAMETERTYPENAME get();
            System::Void set(AbstractionLevels::ABSTRACTIONLEVELNAMERepository::ParameterTypes::PARAMETERTYPENAME value);
        }
        
        public: [System::ComponentModel::Description(L"Feel free you use the same name.")]
        property AbstractionLevels::ABSTRACTIONLEVELNAMERepository::ParameterTypes::ANOTHERPARAMETERTYPENAME ANOTHERPARAMETERTYPENAME
        {
            AbstractionLevels::ABSTRACTIONLEVELNAMERepository::ParameterTypes::ANOTHERPARAMETERTYPENAME get();
            System::Void set(AbstractionLevels::ABSTRACTIONLEVELNAMERepository::ParameterTypes::ANOTHERPARAMETERTYPENAME value);
        }
    };
    
    [System::ComponentModel::Description(L"You can make mark an Object as a specification of another Object. For example if " 
L"the first object is opening then\r\n\tyou can make two more objects door and window" 
L" that both are openings.")]
    public ref class ANOTHEROBJECTNAME : public AbstractionLevels::ABSTRACTIONLEVELNAMERepository::ObjectTypes::OBJECTNAME
    {
        
        private: System::Number^  _nUMBERNAME;
        
        public: [System::ComponentModel::Description(L"You understood that it doesn\'t need to necissaraly be a Number")]
        property System::Number^  NUMBERNAME
        {
            System::Number^  get();
            System::Void set(System::Number^  value);
        }
    };
        }
    }
}
namespace AbstractionLevels {
    namespace ABSTRACTIONLEVELNAMERepository {
        namespace ObjectTypes {
    
    
    inline Text^  OBJECTNAME::PARMATERNAMEOFTEXT::get()
    {
        return this->_pARMATERNAMEOFTEXT;
    }
    inline System::Void OBJECTNAME::PARMATERNAMEOFTEXT::set(Text^  value)
    {
        this->_pARMATERNAMEOFTEXT = value;
    }
    
    inline Point^  OBJECTNAME::PARMATERNAMEOFPoint::get()
    {
        return this->_pARMATERNAMEOFPoint;
    }
    inline System::Void OBJECTNAME::PARMATERNAMEOFPoint::set(Point^  value)
    {
        this->_pARMATERNAMEOFPoint = value;
    }
    
    inline AbstractionLevels::ABSTRACTIONLEVELNAMERepository::ParameterTypes::PARAMETERTYPENAME OBJECTNAME::PARAMTERTYPENAMENAME::get()
    {
        return this->_pARAMTERTYPENAMENAME;
    }
    inline System::Void OBJECTNAME::PARAMTERTYPENAMENAME::set(AbstractionLevels::ABSTRACTIONLEVELNAMERepository::ParameterTypes::PARAMETERTYPENAME value)
    {
        this->_pARAMTERTYPENAMENAME = value;
    }
    
    inline AbstractionLevels::ABSTRACTIONLEVELNAMERepository::ParameterTypes::ANOTHERPARAMETERTYPENAME OBJECTNAME::ANOTHERPARAMETERTYPENAME::get()
    {
        return this->_aNOTHERPARAMETERTYPENAME;
    }
    inline System::Void OBJECTNAME::ANOTHERPARAMETERTYPENAME::set(AbstractionLevels::ABSTRACTIONLEVELNAMERepository::ParameterTypes::ANOTHERPARAMETERTYPENAME value)
    {
        this->_aNOTHERPARAMETERTYPENAME = value;
    }
    
    
    inline System::Number^  ANOTHEROBJECTNAME::NUMBERNAME::get()
    {
        return this->_nUMBERNAME;
    }
    inline System::Void ANOTHEROBJECTNAME::NUMBERNAME::set(System::Number^  value)
    {
        this->_nUMBERNAME = value;
    }
        }
    }
}
namespace AbstractionLevels {
    namespace ABSTRACTIONLEVELNAMERepository {
    using namespace System;
    using namespace AbstractionLevels::ABSTRACTIONLEVELNAMERepository::ObjectTypes;
    using namespace System;
    ref class ABSTRACTIONLEVELNAME;
    
    
    [System::ComponentModel::Description(L"Your description for the ABSTRACTION LEVEL. By the way you don\'t have to change a" 
L"nything\r\nin this file to compile it. It is 100& correct by default and will comp" 
L"ile out of the box.")]
    public ref class ABSTRACTIONLEVELNAME
    {
        
        private: System::Collections::Generic::List<AbstractionLevels::ABSTRACTIONLEVELNAMERepository::ObjectTypes::OBJECTNAME^ >^  _oBJECTNAMECollection;
        
        private: System::Collections::Generic::List<AbstractionLevels::ABSTRACTIONLEVELNAMERepository::ObjectTypes::ANOTHEROBJECTNAME^ >^  _aNOTHEROBJECTNAMECollection;
        
        public: [System::ComponentModel::Description(L"This is your first Object that has parameters. The PARAMETERTYPE can be a Paramet" 
L"er from above \r\n\tor a standard one like Point, Text, Curve, Number, ...")]
        property System::Collections::Generic::List<AbstractionLevels::ABSTRACTIONLEVELNAMERepository::ObjectTypes::OBJECTNAME^ >^  OBJECTNAMECollection
        {
            System::Collections::Generic::List<AbstractionLevels::ABSTRACTIONLEVELNAMERepository::ObjectTypes::OBJECTNAME^ >^  get();
            System::Void set(System::Collections::Generic::List<AbstractionLevels::ABSTRACTIONLEVELNAMERepository::ObjectTypes::OBJECTNAME^ >^  value);
        }
        
        public: [System::ComponentModel::Description(L"You can make mark an Object as a specification of another Object. For example if " 
L"the first object is opening then\r\n\tyou can make two more objects door and window" 
L" that both are openings.")]
        property System::Collections::Generic::List<AbstractionLevels::ABSTRACTIONLEVELNAMERepository::ObjectTypes::ANOTHEROBJECTNAME^ >^  ANOTHEROBJECTNAMECollection
        {
            System::Collections::Generic::List<AbstractionLevels::ABSTRACTIONLEVELNAMERepository::ObjectTypes::ANOTHEROBJECTNAME^ >^  get();
            System::Void set(System::Collections::Generic::List<AbstractionLevels::ABSTRACTIONLEVELNAMERepository::ObjectTypes::ANOTHEROBJECTNAME^ >^  value);
        }
    };
    }
}
namespace AbstractionLevels {
    namespace ABSTRACTIONLEVELNAMERepository {
    
    
    inline System::Collections::Generic::List<AbstractionLevels::ABSTRACTIONLEVELNAMERepository::ObjectTypes::OBJECTNAME^ >^  ABSTRACTIONLEVELNAME::OBJECTNAMECollection::get()
    {
        return this->_oBJECTNAMECollection;
    }
    inline System::Void ABSTRACTIONLEVELNAME::OBJECTNAMECollection::set(System::Collections::Generic::List<AbstractionLevels::ABSTRACTIONLEVELNAMERepository::ObjectTypes::OBJECTNAME^ >^  value)
    {
    }
    
    inline System::Collections::Generic::List<AbstractionLevels::ABSTRACTIONLEVELNAMERepository::ObjectTypes::ANOTHEROBJECTNAME^ >^  ABSTRACTIONLEVELNAME::ANOTHEROBJECTNAMECollection::get()
    {
        return this->_aNOTHEROBJECTNAMECollection;
    }
    inline System::Void ABSTRACTIONLEVELNAME::ANOTHEROBJECTNAMECollection::set(System::Collections::Generic::List<AbstractionLevels::ABSTRACTIONLEVELNAMERepository::ObjectTypes::ANOTHEROBJECTNAME^ >^  value)
    {
    }
    }
}
namespace AbstractionLevels {
    namespace ANOTHERABSTRACTIONLEVELNAMERepository {
        namespace ParameterTypes {
    using namespace System;
    using namespace System;
    
        }
    }
}
namespace AbstractionLevels {
    namespace ANOTHERABSTRACTIONLEVELNAMERepository {
        namespace ParameterTypes {
        }
    }
}
namespace AbstractionLevels {
    namespace ANOTHERABSTRACTIONLEVELNAMERepository {
        namespace ObjectTypes {
    using namespace System;
    using namespace AbstractionLevels::ANOTHERABSTRACTIONLEVELNAMERepository::ParameterTypes;
    using namespace SemIOLibrary::Parameters;
    using namespace SemIOLibrary::Parameters::GeometryParameters;
    using namespace SemIOLibrary::Parameters::GeometryParameters::Surfaces;
    using namespace SemIOLibrary::Parameters::GeometryParameters::Solids;
    using namespace SemIOLibrary::Parameters::GeometryParameters::Points;
    using namespace SemIOLibrary::Parameters::GeometryParameters::Curves;
    using namespace SemIOLibrary::Parameters::BaseParameters;
    using namespace SemIOLibrary::Parameters::BaseParameters::Numbers;
    using namespace System;
    ref class OBJECTNAME;
    
    
    [System::ComponentModel::Description(L"I don\'t need any special parameters for this abstraction level. Therefore I don\'t" 
L" need any Parameters.")]
    public ref class OBJECTNAME
    {
        
        private: Surface^  _sURFACENAME;
        
        private: Solid^  _sOLIDNAME;
        
        public: [System::ComponentModel::Description(L"")]
        property Surface^  SURFACENAME
        {
            Surface^  get();
            System::Void set(Surface^  value);
        }
        
        public: [System::ComponentModel::Description(L"Last one and now go to compile this code. It will actually compile just like this" 
L"!")]
        property Solid^  SOLIDNAME
        {
            Solid^  get();
            System::Void set(Solid^  value);
        }
    };
        }
    }
}
namespace AbstractionLevels {
    namespace ANOTHERABSTRACTIONLEVELNAMERepository {
        namespace ObjectTypes {
    
    
    inline Surface^  OBJECTNAME::SURFACENAME::get()
    {
        return this->_sURFACENAME;
    }
    inline System::Void OBJECTNAME::SURFACENAME::set(Surface^  value)
    {
        this->_sURFACENAME = value;
    }
    
    inline Solid^  OBJECTNAME::SOLIDNAME::get()
    {
        return this->_sOLIDNAME;
    }
    inline System::Void OBJECTNAME::SOLIDNAME::set(Solid^  value)
    {
        this->_sOLIDNAME = value;
    }
        }
    }
}
namespace AbstractionLevels {
    namespace ANOTHERABSTRACTIONLEVELNAMERepository {
    using namespace System;
    using namespace AbstractionLevels::ANOTHERABSTRACTIONLEVELNAMERepository::ObjectTypes;
    using namespace AbstractionLevels::ABSTRACTIONLEVELNAMERepository;
    using namespace System;
    ref class ANOTHERABSTRACTIONLEVELNAME;
    
    
    [System::ComponentModel::Description(L"Speaking of specification. Also abstraction levels can be a more in depth version" 
L" of another abstraction level.\r\na masterplan is more fuzzy than a roomplan. So y" 
L"ou need to say which one is more precise.")]
    public ref class ANOTHERABSTRACTIONLEVELNAME : public AbstractionLevels::ABSTRACTIONLEVELNAMERepository::ABSTRACTIONLEVELNAME
    {
        
        private: System::Collections::Generic::List<AbstractionLevels::ANOTHERABSTRACTIONLEVELNAMERepository::ObjectTypes::OBJECTNAME^ >^  _oBJECTNAMECollection;
        
        public: [System::ComponentModel::Description(L"I don\'t need any special parameters for this abstraction level. Therefore I don\'t" 
L" need any Parameters.")]
        property System::Collections::Generic::List<AbstractionLevels::ANOTHERABSTRACTIONLEVELNAMERepository::ObjectTypes::OBJECTNAME^ >^  OBJECTNAMECollection
        {
            System::Collections::Generic::List<AbstractionLevels::ANOTHERABSTRACTIONLEVELNAMERepository::ObjectTypes::OBJECTNAME^ >^  get();
            System::Void set(System::Collections::Generic::List<AbstractionLevels::ANOTHERABSTRACTIONLEVELNAMERepository::ObjectTypes::OBJECTNAME^ >^  value);
        }
    };
    }
}
namespace AbstractionLevels {
    namespace ANOTHERABSTRACTIONLEVELNAMERepository {
    
    
    inline System::Collections::Generic::List<AbstractionLevels::ANOTHERABSTRACTIONLEVELNAMERepository::ObjectTypes::OBJECTNAME^ >^  ANOTHERABSTRACTIONLEVELNAME::OBJECTNAMECollection::get()
    {
        return this->_oBJECTNAMECollection;
    }
    inline System::Void ANOTHERABSTRACTIONLEVELNAME::OBJECTNAMECollection::set(System::Collections::Generic::List<AbstractionLevels::ANOTHERABSTRACTIONLEVELNAMERepository::ObjectTypes::OBJECTNAME^ >^  value)
    {
    }
    }
}
