// TODO Autogenerate
using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.Remoting.Messaging;
using System.Text;
using System.Threading.Tasks;
using System.Xml.Linq;
using Google.Protobuf;
using Grasshopper.Kernel.Types;
using Rhino.Geometry;
using Semio.Model.V1;
using Encoding = System.Text.Encoding;

namespace Semio.UI.Grasshopper.Goos
{
    public class ParameterGoo : SemioGoo<Parameter>
    {
        public ParameterGoo()
        {
            Value = new Parameter();
        }

        public ParameterGoo(Parameter parameter)
        {
            Value = parameter;
        }

        public override IGH_Goo Duplicate() => new ParameterGoo(Value.Clone());
        public override string TypeName => Parameter.Descriptor.FullName;
        public override string TypeDescription => Parameter.Descriptor.Declaration.LeadingComments;

        public override bool CastFrom(object source)
        {
            if (source == null) return false;

            switch (source)
            {
                case GH_String text:
                    try
                    {
                        Value = Parameter.Parser.ParseJson(text.Value);
                    }
                    catch (Exception e)
                    {
                        try
                        {
                            Value = Parameter.Parser.ParseFrom(ByteString.CopyFrom(text.Value, Encoding.Default));
                        }
                        catch (Exception exception)
                        {
                            return false;
                        }
                    }
                    return true;
                default:
                    return false;
            }
        }
    }
}