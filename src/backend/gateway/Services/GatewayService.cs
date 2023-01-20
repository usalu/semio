using Grpc.Core;
using Semio.Assembler.V1;
using GatewayServiceBase = Semio.Gateway.V1.GatewayService.GatewayServiceBase;
using Semio.Model.V1;


namespace Semio.Backend.Gateway.Services
{
    public class GatewayService : GatewayServiceBase
    {
        private readonly ILogger<GatewayService> _logger;
        public GatewayService(ILogger<GatewayService> logger)
        {
            _logger = logger;
        }

        public override Task<Design> LayoutDesign(LayoutDesignRequest request, ServerCallContext context)
        {
            return Task.FromResult(new Design()
            {
                Elements = { new Element()
                {
                    Representations =
                    {
                        new Representation()
                        {
                            Text = "I am a demo representation"
                        }
                    }
                } }
            });
        }
    }
}