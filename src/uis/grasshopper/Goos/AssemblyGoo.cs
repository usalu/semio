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
    public class AssemblyGoo : SemioGoo<Assembly>
    {
        public AssemblyGoo()
        {
            Value = new Assembly();
        }

        public AssemblyGoo(Assembly assembly)
        {
            Value = assembly;
        }
        public override IGH_Goo Duplicate() => new AssemblyGoo(Value.Clone());
        public override string TypeName => Assembly.Descriptor.FullName;
        public override string TypeDescription => Assembly.Descriptor.Declaration.LeadingComments;

        public override bool CastFrom(object source)
        {
            if (source == null) return false;

            switch (source)
            {
                case GH_String text:
                    try
                    {
                        Value = Assembly.Parser.ParseJson(text.Value);
                        return true;
                    }
                    catch (Exception e)
                    {
                        try
                        {
                            Value = Assembly.Parser.ParseFrom(ByteString.CopyFrom(text.Value, Encoding.Default));
                            return true;
                        }
                        catch (Exception exception)
                        {
                        }
                    }
                    return false;
                case SobjectGoo sobject:
                    Value = new Assembly()
                    {
                        SobjectId = sobject.Value.Id
                    };
                    return true;
                default:
                    return false;
            }
        }
    }
}